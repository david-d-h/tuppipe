use std::marker::PhantomData;

use crate::Pipe;

#[inline]
pub const fn also<'value, P, T>(pipe: P) -> Also<'value, P, T>
where
    P: Pipe<&'value T, Output = ()>,
{
    Also {
        _t: PhantomData,
        pipe,
    }
}

#[inline]
pub fn also_mut<'value, P, T>(pipe: P) -> AlsoMut<'value, P, T>
where
    P: Pipe<&'value mut T, Output = ()>,
{
    AlsoMut {
        _t: PhantomData,
        pipe,
    }
}

pub struct Also<'value, P, T>
where
    P: Pipe<&'value T, Output = ()>,
{
    _t: PhantomData<&'value T>,
    pipe: P,
}

impl<'value, P, T> Pipe<&'value T> for Also<'value, P, T>
where
    P: for<'local> Pipe<&'local T, Output = ()>,
{
    type Output = &'value T;

    #[inline]
    fn complete(self, value: &'value T) -> Self::Output {
        self.pipe.complete(value);
        value
    }
}

impl<'value, P, T> Pipe<&'value mut T> for Also<'value, P, T>
where
    P: for<'local> Pipe<&'local T, Output = ()>,
{
    type Output = &'value mut T;

    #[inline]
    fn complete(self, value: &'value mut T) -> Self::Output {
        self.pipe.complete(value);
        value
    }
}

impl<P, T> Pipe<T> for Also<'_, P, T>
where
    P: for<'local> Pipe<&'local T, Output = ()>,
{
    type Output = T;

    #[inline]
    fn complete(self, value: T) -> Self::Output {
        self.pipe.complete(&value);
        value
    }
}

pub struct AlsoMut<'value, P, T>
where
    P: Pipe<&'value mut T, Output = ()>,
{
    _t: PhantomData<&'value mut T>,
    pipe: P,
}

impl<'value, P, T> Pipe<&'value mut T> for AlsoMut<'value, P, T>
where
    P: for<'local> Pipe<&'local mut T, Output = ()>,
{
    type Output = &'value mut T;

    #[inline]
    fn complete(self, value: &'value mut T) -> Self::Output {
        self.pipe.complete(value);
        value
    }
}

impl<P, T> Pipe<T> for AlsoMut<'_, P, T>
where
    P: for<'local> Pipe<&'local mut T, Output = ()>,
{
    type Output = T;

    #[inline]
    fn complete(self, mut value: T) -> Self::Output {
        self.pipe.complete(&mut value);
        value
    }
}
