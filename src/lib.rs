pub use paste;

pub struct PartialPipe<T>(T);

/// The [`pipe`] function makes a partial pipe by wrapping a generic `T` in a [`PartialPipe`].
///
/// ### Example
///
/// ```rs
/// const fn add_one(to: i32) -> i32 {
///     to + 1
/// }
///
/// assert_eq!(3, pipe(1) >> (add_one, add_one));
/// ```
#[inline]
pub const fn pipe<T>(inner: T) -> PartialPipe<T> {
    PartialPipe(inner)
}

impl<P, T, R> std::ops::Shr<P> for PartialPipe<T>
where
    P: Pipe<T, Output = R>,
{
    type Output = R;

    #[inline]
    fn shr(self, pipe: P) -> Self::Output {
        pipe.complete(self.0)
    }
}

pub trait Pipe<T> {
    type Output;

    /// Complete a given pipe.
    fn complete(self, value: T) -> Self::Output;
}

impl<F: FnOnce(T) -> R, T, R> Pipe<T> for F {
    type Output = R;

    #[inline]
    fn complete(self, value: T) -> Self::Output {
        self(value)
    }
}

#[macro_export]
macro_rules! generate_pipe_ntup_impl {
    (
        $(($pF:ident -> $($pN:ident),+ -> $pL:ident)),* $(,)?
    ) => ($crate::paste::paste! {
        $($crate::generate_pipe_ntup_impl!(@gen_item
                [impl<T, $pF, $($pN),+, $pL, [<R $pF>], $([<R $pN>]),+, [<R $pL>]> $crate::Pipe<T> for ($pF, $($pN),+, $pL)]
                [$pF: $crate::Pipe<T, Output = [<R $pF>]>,]
                ($pF, $($pN),+, $pL)
                [{
                    type Output = [<R $pL>];

                    fn complete(self, value: T) -> Self::Output {
                        let ([<$pF:lower>], $([<$pN:lower>]),+, [<$pL:lower>]) = self;
                        let item = [<$pF:lower>].complete(value);
                        $(
                        let item = [<$pN:lower>].complete(item);
                        )+
                        [<$pL:lower>].complete(item)
                    }
                }]
        );)*
    });
    (@gen_item [$($pre_clause:tt)+] [$($buffer:tt)+] ($pL:ident, $pC:ident $(, $pN:ident)*) [$($post_clause:tt)+]) => ($crate::paste::paste! {
        $crate::generate_pipe_ntup_impl!(@gen_item
            [$($pre_clause)+]
            [$($buffer)+ $pC: $crate::Pipe<[<R $pL>], Output = [<R $pC>]>,]
            ($pC $(, $pN)*)
            [$($post_clause)+]
        );
    });
    (@gen_item [$($pre_clause:tt)+] [$($buffer:tt)+] ($($residual:tt)*) [$($post_clause:tt)+]) => {
        $($pre_clause)+
        where $($buffer)+
        $($post_clause)+
    };
}

impl<T, P1, P2, PR1, PR2> Pipe<T> for (P1, P2)
where
    P1: Pipe<T, Output = PR1>,
    P2: Pipe<PR1, Output = PR2>,
{
    type Output = PR2;

    #[inline]
    fn complete(self, value: T) -> Self::Output {
        self.1.complete(self.0.complete(value))
    }
}

generate_pipe_ntup_impl!(
    (P1 -> P2 -> P3),
    (P1 -> P2, P3 -> P4),
    (P1 -> P2, P3, P4 -> P5),
    (P1 -> P2, P3, P4, P5 -> P6),
    (P1 -> P2, P3, P4, P5, P6 -> P7),
    (P1 -> P2, P3, P4, P5, P6, P7 -> P8),
    (P1 -> P2, P3, P4, P5, P6, P7, P8 -> P9),
    (P1 -> P2, P3, P4, P5, P6, P7, P8, P9 -> P10),
    (P1 -> P2, P3, P4, P5, P6, P7, P8, P9, P10 -> P11),
    (P1 -> P2, P3, P4, P5, P6, P7, P8, P9, P10, P11 -> P12),
    (P1 -> P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12 -> P13),
    (P1 -> P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13 -> P14),
    (P1 -> P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14 -> P15),
    (P1 -> P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15 -> P16),
);

#[cfg(test)]
mod tests {
    use super::*;

    const fn add_one(to: i32) -> i32 {
        to + 1
    }

    #[test]
    fn it_works() {
        assert_eq!(2, pipe(0) >> (add_one, add_one));
    }

    #[test]
    fn a_closure_works_as_a_pipe() {
        assert_eq!(2, pipe(0) >> (|x| x + 1, |x| x + 1));
    }

    #[test]
    fn tuples_of_pipes_can_be_infinitely_nested() {
        assert_eq!(
            6,
            pipe(0) >> (add_one, add_one, (add_one, add_one, (add_one, add_one)))
        );
    }
}
