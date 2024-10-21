#![doc = include_str!("../README.md")]
#![feature(
    fn_traits,
    macro_metavar_expr,
    decl_macro,
    unboxed_closures,
    associated_type_defaults,
    auto_traits,
    negative_impls
)]

pub mod prelude;

pub mod also;
pub mod on;
pub mod take_if;
pub mod void;

pub use paste::paste;

/// This [`PartialPipeline`] struct is a necessary wrapper around a generic `T`, to implement
/// a foreign trait (the pipe operator of choice, `Shr`) for any arbitrary type.
///
/// ### Example
///
/// ```rust
/// use tuppipe::pipe;
///
/// assert_eq!(1, pipe(0) >> |x| x + 1);
/// ```
pub struct PartialPipeline<'r, 'env, T>(T, std::marker::PhantomData<&'r &'env ()>);

/// The [`PartialIgnoredPipeline`] struct is essentially the same as the [`PartialPipeline`]
/// struct, however with this struct the result of your pipeline will be ignored, and you will
/// instead receive the unit type `()` as a result.
///
/// This is meant to fix the problem where: in pipelines of which you don't care about the result
/// the compiler will complain about you not using the returned value. This happens because the
/// [`Shr`] trait this crate uses on the `PartialPipeline` types has the `#[must_use]` attribute
/// on the method we *actually implement*.
///
/// You could just do `let _ = pipeline`, but nobody wants that.
///
/// ### Example
///
/// ```rust
/// use tuppipe::{ignore, pipe};
///
/// ignore(0) >> |x| x + 1;
///
/// // instead of
///
/// let _ = pipe(0) >> |x| x + 1;
/// ```
///
/// [`Shr`]: std::ops::Shr
pub struct PartialIgnoredPipeline<'r, 'env, T>(T, std::marker::PhantomData<&'r &'env ()>);

/// The [`pipe`] function makes a partial pipeline by wrapping a generic `T` in a [`PartialPipeline`].
///
/// ### Example
///
/// ```rust
/// use tuppipe::pipe;
///
/// const fn add_one(to: i32) -> i32 {
///     to + 1
/// }
///
/// assert_eq!(3, pipe(1) >> (add_one, add_one));
/// ```
#[inline]
pub fn pipe<'r, 'env, T>(inner: T) -> PartialPipeline<'r, 'env, T> {
    PartialPipeline(inner, std::marker::PhantomData)
}

/// The [`ignore`] function makes a partial **ignored** pipeline. This means that no matter
/// the result of your pipeline, the result of it will be ignored, and you will receive the
/// unit type `()` instead.
///
/// See also: [`PartialIgnoredPipeline`]
#[inline]
pub fn ignore<'r, 'env, T>(inner: T) -> PartialIgnoredPipeline<'r, 'env, T> {
    PartialIgnoredPipeline(inner, std::marker::PhantomData)
}

impl<'r, 'env, T> PartialPipeline<'r, 'env, T> {
    /// Transform a regular [`PartialPipeline`] into a [`PartialIgnoredPipeline`].
    #[inline]
    pub fn ignore(self) -> PartialIgnoredPipeline<'r, 'env, T> {
        PartialIgnoredPipeline(self.0, self.1)
    }
}

/// The [`Pipe`] trait has to be implemented by, well... pipes.
/// Anything that implements this is usable as a pipe where the
/// item in the pipeline (at said pipe's position) is of type `T`.
///
/// ### Implementing your own pipe
///
/// The [`Pipe`] trait is public, meaning you can totally implement
/// your own pipes. Here is an example of how.
///
/// ```rust
/// use tuppipe::*;
///
/// struct Subtractor<const N: i32>;
///
/// impl<const N: i32> Pipe<'_, '_, i32> for Subtractor<N> {
///     type Output = i32;
///
///     fn complete(self, value: i32) -> Self::Output {
///         value - N
///     }
/// }
///
/// assert_eq!(-2, pipe(0) >> Subtractor::<2>)
/// ```
pub trait Pipe<'r, 'env, T, _Bound = &'r &'env ()> {
    type Output;

    /// Complete a given pipe.
    fn complete(self, value: T) -> Self::Output;
}

impl<'r, 'env, P, T, R> std::ops::Shr<P> for PartialPipeline<'r, 'env, T>
where
    P: Pipe<'r, 'env, T, Output = R>,
{
    type Output = R;

    #[inline]
    fn shr(self, pipe: P) -> Self::Output {
        pipe.complete(self.0)
    }
}

impl<'r, 'env, P, T, R> std::ops::Shr<P> for PartialIgnoredPipeline<'r, 'env, T>
where
    P: Pipe<'r, 'env, T, Output = R>,
{
    type Output = ();

    #[inline]
    fn shr(self, pipe: P) -> Self::Output {
        pipe.complete(self.0);
    }
}

#[cfg(feature = "fn-pipes")]
pub auto trait MarkerFnPipe {}

#[cfg(feature = "fn-pipes")]
impl<'r, 'env, F: FnOnce(T) -> R, T, R> Pipe<'r, 'env, T> for F
where
    F: MarkerFnPipe,
{
    type Output = R;

    #[inline]
    fn complete(self, value: T) -> Self::Output {
        self(value)
    }
}

macro_rules! generate_pipe_ntup_impl {
    (
        $(($pF:ident -> $($pN:ident),+ -> $pL:ident)),* $(,)?
    ) => (::paste::paste! {
        $(generate_pipe_ntup_impl!(@gen_item
                [impl<'r, 'env, T, $pF, $($pN),+, $pL, [<R $pF>], $([<R $pN>]),+, [<R $pL>]> $crate::Pipe<'r, 'env, T> for ($pF, $($pN),+, $pL)]
                [$pF: $crate::Pipe<'r, 'env, T, Output = [<R $pF>]>,]
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
    (@gen_item [$($pre_clause:tt)+] [$($buffer:tt)+] ($pL:ident, $pC:ident $(, $pN:ident)*) [$($post_clause:tt)+]) => (::paste::paste! {
        generate_pipe_ntup_impl!(@gen_item
            [$($pre_clause)+]
            [$($buffer)+ $pC: $crate::Pipe<'r, 'env, [<R $pL>], Output = [<R $pC>]>,]
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

impl<'r, 'env, T, P1, P2, RP1, RP2> Pipe<'r, 'env, T> for (P1, P2)
where
    P1: Pipe<'r, 'env, T, Output = RP1>,
    P2: Pipe<'r, 'env, RP1, Output = RP2>,
{
    type Output = RP2;

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
        assert_eq!(1, pipe(0) >> |x| x + 1);
    }

    #[test]
    fn tuples_of_pipes_can_be_infinitely_nested() {
        assert_eq!(
            6,
            pipe(0) >> (add_one, add_one, (add_one, add_one, (add_one, add_one)))
        );
    }

    #[test]
    fn custom_pipe_implementation() {
        struct Subtractor<const N: i32>;

        impl<const N: i32> Pipe<'_, '_, i32> for Subtractor<N> {
            type Output = i32;

            fn complete(self, value: i32) -> Self::Output {
                value - N
            }
        }

        assert_eq!(-2, pipe(0) >> Subtractor::<2>);
    }

    #[test]
    fn desugared_method() {
        struct Int32(i32);

        impl Int32 {
            fn add_one(self) -> Self {
                Self(self.0 + 1)
            }
        }

        assert_eq!(1i32, pipe(Int32(0)) >> (Int32::add_one, |Int32(n)| n));
    }

    #[test]
    fn ignore_pipeline_result() {
        assert_eq!((), pipe(0).ignore() >> |x| x + 1);
    }
}
