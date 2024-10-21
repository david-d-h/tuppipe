use std::marker::PhantomData;

use crate::{MarkerFnPipe, Pipe};

use ghost::phantom;

mod private {
    pub trait Sealed<T, R> {}
}

impl<'r, 'env: 'r, P: Pipe<'r, 'env, T, Output = R>, T, R> private::Sealed<T, R> for P {}

pub trait Voidable<'env, T, R>:
    private::Sealed<T, R> + for<'r> Pipe<'r, 'env, T, Output = R> + Sized
{
    #[inline]
    fn void(self) -> VoidInner<'env, Self, T, R> {
        VoidInner {
            _t: PhantomData,
            pipe: self,
        }
    }
}

impl<'env, P, T, R> Voidable<'env, T, R> for P where P: for<'r> Pipe<'r, 'env, T, Output = R> {}

pub struct VoidInner<'env, P, T, R>
where
    P: for<'r> Pipe<'r, 'env, T, Output = R>,
{
    _t: PhantomData<for<'r> fn(&'r mut T) -> &'r &'env ()>,
    pipe: P,
}

#[phantom]
#[allow(non_camel_case_types)]
pub struct void<'env, T, Mode = fn(T)>;

#[cfg(feature = "fn-pipes")]
impl<T, Mode> !MarkerFnPipe for void<'_, T, Mode> {}

impl<'r, 'env, T> Pipe<'r, 'env, &'r T> for void<'env, T, fn(&T)> {
    type Output = ();

    #[inline]
    fn complete(self, _value: &'r T) -> Self::Output {}
}

impl<'r, 'env, T> Pipe<'r, 'env, &'r mut T> for void<'env, T, fn(&mut T)> {
    type Output = ();

    #[inline]
    fn complete(self, _value: &'r mut T) -> Self::Output {}
}

impl<'r, 'env, T> Pipe<'r, 'env, T> for void<'env, T, fn(T)> {
    type Output = ();

    #[inline]
    fn complete(self, _value: T) -> Self::Output {}
}

impl<'env, P, T, R, Mode> FnOnce<(P,)> for void<'env, T, Mode>
where
    P: for<'r> Pipe<'r, 'env, T, Output = R>,
{
    type Output = VoidInner<'env, P, T, R>;

    #[inline]
    extern "rust-call" fn call_once(self, args: (P,)) -> Self::Output {
        VoidInner {
            _t: PhantomData,
            pipe: args.0,
        }
    }
}

impl<'r2, 'env, P, T, R> Pipe<'r2, 'env, T> for VoidInner<'env, P, T, R>
where
    P: for<'r> Pipe<'r, 'env, T, Output = R>,
{
    type Output = ();

    #[inline]
    fn complete(self, value: T) -> Self::Output {
        _ = self.pipe.complete(value);
    }
}

#[test]
#[cfg(test)]
fn it_works() {
    let _it: () = crate::pipe(1) >> void::<_, fn(i32)>((void, void)).void();
}
