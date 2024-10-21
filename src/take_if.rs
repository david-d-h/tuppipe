use std::{borrow::Borrow, marker::PhantomData};

use crate::Pipe;

#[inline]
pub const fn take_if<'env, P, T>(pipe: P) -> TakeIf<'env, P, T>
where
    P: for<'r> Pipe<'r, 'env, &'r T, Output = bool>,
{
    TakeIf {
        _t: PhantomData,
        pipe,
    }
}

pub struct TakeIf<'env, P, T>
where
    P: for<'r> Pipe<'r, 'env, &'r T, Output = bool>,
{
    _t: PhantomData<for<'r> fn(&'r T) -> &'r &'env ()>,
    pipe: P,
}

impl<'r2, 'env, P, T> Pipe<'r2, 'env, T> for TakeIf<'env, P, T>
where
    P: for<'r> Pipe<'r, 'env, &'r T, Output = bool>,
    T: Borrow<T>,
{
    type Output = Option<T>;

    #[inline]
    fn complete(self, value: T) -> Self::Output {
        self.pipe.complete(value.borrow()).then_some(value)
    }
}

#[test]
#[cfg(test)]
fn it_works() {
    use crate::prelude::*;

    let it = take_if((void::<_, fn(&_)>, |_x| false)).complete(());

    assert_eq!(it, None);
}
