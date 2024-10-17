use crate::Pipe;

use ghost::phantom;

mod private {
    pub trait Sealed<T, R> {}
}

impl<P: Pipe<T, Output = R>, T, R> private::Sealed<T, R> for P {}

pub(crate) trait NotVoid<T, R>: private::Sealed<T, R> {}

default impl<F: FnOnce(T) -> R, T, R> NotVoid<T, R> for F {}

impl<T, R> !NotVoid<T, R> for void<T, R> {}

pub trait Voidable<T, R>: private::Sealed<T, R> + Pipe<T, Output = R> + Sized {
    #[inline]
    fn void(self) -> VoidInner<Self, T, R> {
        VoidInner(self, core::marker::PhantomData)
    }
}

impl<P: Pipe<T, Output = R>, T, R> Voidable<T, R> for P {}

pub struct VoidInner<P: Pipe<T, Output = R>, T, R>(P, core::marker::PhantomData<(T, R)>);

#[phantom]
#[allow(non_camel_case_types)]
pub struct void<T, R>;

impl<T> Pipe<T> for void<T, ()> {
    type Output = ();

    #[inline]
    fn complete(self, _value: T) -> <Self as Pipe<T>>::Output {}
}

impl<P: Pipe<T, Output = R>, T, R> FnOnce<(P,)> for void<T, R> {
    type Output = VoidInner<P, T, R>;

    #[inline]
    extern "rust-call" fn call_once(self, args: (P,)) -> Self::Output {
        VoidInner(args.0, core::marker::PhantomData)
    }
}

impl<P: Pipe<T, Output = R>, T, R> Pipe<T> for VoidInner<P, T, R> {
    type Output = ();

    #[inline]
    fn complete(self, value: T) -> Self::Output {
        self.0.complete(value);
    }
}

#[test]
fn test() {
    let _it: () = crate::pipe(1) >> void(void).void();
}
