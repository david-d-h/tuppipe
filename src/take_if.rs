use std::marker::PhantomData;

use crate::Pipe;

#[inline]
pub const fn take_if<'value, P, T>(pipe: P) -> TakeIf<'value, P, T>
where
    P: Pipe<&'value T, Output = bool>,
{
    TakeIf {
        _t: PhantomData,
        pipe,
    }
}

pub struct TakeIf<'value, P, T>
where
    P: Pipe<&'value T, Output = bool>,
{
    _t: PhantomData<&'value T>,
    pipe: P,
}

impl<'value, P, T> Pipe<&'value T> for TakeIf<'value, P, T>
where
    P: for<'local> Pipe<&'local T, Output = bool>,
{
    type Output = Option<&'value T>;

    #[inline]
    fn complete(self, value: &'value T) -> Self::Output {
        self.pipe.complete(value).then_some(value)
    }
}

impl<'value, P, T> Pipe<&'value mut T> for TakeIf<'value, P, T>
where
    P: for<'local> Pipe<&'local T, Output = bool>,
{
    type Output = Option<&'value mut T>;

    #[inline]
    fn complete(self, value: &'value mut T) -> Self::Output {
        self.pipe.complete(value).then_some(value)
    }
}

impl<P, T> Pipe<T> for TakeIf<'_, P, T>
where
    P: for<'local> Pipe<&'local T, Output = bool>,
{
    type Output = Option<T>;

    #[inline]
    fn complete(self, value: T) -> Self::Output {
        self.pipe.complete(&value).then_some(value)
    }
}
