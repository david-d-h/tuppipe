mod private {
    pub trait Sealed {}
}

pub trait InferSelf<F, S>: private::Sealed {
    type Inferred = S;
}

pub type Infer<F, S> = std::marker::PhantomData<(F, S)>;

impl<F, S> private::Sealed for Infer<F, S> {}

impl<F, S> InferSelf<F, S> for Infer<F, S> {}

/// A macro used to generate pipes that do partial application.
pub macro on {
    (($($pre:expr,)* _ $(, $post:expr)* $(,)?) -> $method:expr) => ($crate::paste!({
        #[allow(non_camel_case_types)]
        struct __on_auto_generated_partial_application<S, F, $([<APre${index()}${ignore($pre)}>],)* I, $([<APost${index()}${ignore($post)}>],)* R>
        where
            I: $crate::on::InferSelf<F, S, Inferred = S>,
            F: ::core::ops::FnOnce($([<APre${index()}${ignore($pre)}>],)* S, $([<APost${index()}${ignore($post)}>],)*) -> R,
        {
            f: F,
            args: ($([<APre${index()}${ignore($pre)}>],)* I, $([<APost${index()}${ignore($post)}>],)*),
            _phantom: ::core::marker::PhantomData<(S, R)>,
        }

        impl<S, F, $([<APre${index()}${ignore($pre)}>],)* I, $([<APost${index()}${ignore($post)}>],)* R>
        $crate::Pipe<S> for __on_auto_generated_partial_application<S, F, $([<APre${index()}${ignore($pre)}>],)* I, $([<APost${index()}${ignore($post)}>],)* R>
        where
            I: $crate::on::InferSelf<F, S, Inferred = S>,
            F: ::core::ops::FnOnce($([<APre${index()}${ignore($pre)}>],)* I::Inferred, $([<APost${index()}${ignore($post)}>],)*) -> R,
        {
            type Output = R;

            #[inline]
            fn complete(self, value: S) -> Self::Output {
                let (
                    $([<apre${index()}${ignore($pre)}>],)*
                    _inferred,
                    $([<apost${index()}${ignore($post)}>],)*
                ) = self.args;

                self.f.call_once((
                    $([<apre${index()}${ignore($pre)}>],)*
                    value,
                    $([<apost${index()}${ignore($post)}>],)*
                ))
            }
        }

        __on_auto_generated_partial_application {
            f: $method,
            args: ($($pre,)* ::core::marker::PhantomData::<(_, _)>, $($post,)*),
            _phantom: ::core::marker::PhantomData::<(_, _)>,
        }
    })),
}
