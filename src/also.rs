use std::{
    borrow::{Borrow, BorrowMut},
    marker::PhantomData,
};

use crate::{MarkerFnPipe, Pipe};

#[inline]
pub const fn also<'env, P, T>(pipe: P) -> Also<'env, P, T>
where
    P: for<'r> Pipe<'r, 'env, &'r T, Output = ()>,
{
    Also {
        _t: PhantomData,
        pipe,
    }
}

#[inline]
pub fn also_mut<'env, P, T>(pipe: P) -> AlsoMut<'env, P, T>
where
    P: for<'r> Pipe<'r, 'env, &'r mut T, Output = ()>,
{
    AlsoMut {
        _t: PhantomData,
        pipe,
    }
}

pub struct Also<'env, P, T>
where
    P: for<'r> Pipe<'r, 'env, &'r T, Output = ()>,
{
    _t: PhantomData<for<'r> fn(&'r mut T) -> &'r &'env ()>,
    pipe: P,
}

impl<'r2, 'env, P, T> Pipe<'r2, 'env, T> for Also<'env, P, T>
where
    P: for<'r> Pipe<'r, 'env, &'r T, Output = ()>,
    T: Borrow<T>,
{
    type Output = T;

    #[inline]
    fn complete(self, value: T) -> Self::Output {
        let ref_ = <T as Borrow<T>>::borrow(&value);
        self.pipe.complete(ref_);
        value
    }
}

pub struct AlsoMut<'env, P, T>
where
    P: for<'r> Pipe<'r, 'env, &'r mut T, Output = ()>,
{
    _t: PhantomData<for<'r> fn(&'r mut T) -> &'r &'env ()>,
    pipe: P,
}

impl<'r2, 'env, P, T> Pipe<'r2, 'env, T> for AlsoMut<'env, P, T>
where
    P: for<'r> Pipe<'r, 'env, &'r mut T, Output = ()>,
    T: BorrowMut<T>,
{
    type Output = T;

    fn complete(self, mut value: T) -> Self::Output {
        let ref_ = <T as BorrowMut<T>>::borrow_mut(&mut value);
        self.pipe.complete(ref_);
        value
    }
}

#[cfg(feature = "fn-pipes")]
impl<P, T> !MarkerFnPipe for Also<'_, P, T> {}

#[cfg(feature = "fn-pipes")]
impl<P, T> !MarkerFnPipe for AlsoMut<'_, P, T> {}

#[test]
#[cfg(test)]
fn test() {
    use crate::prelude::*;

    let one = also_mut((void::<_, fn(&mut _)>, |_it| ())).complete(1i32);

    assert_eq!(one, 1);
}
