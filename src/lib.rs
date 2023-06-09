#![no_std]

pub use auto_delegate_impl::{delegate, Delegate};

macro_rules! expand_macro_maker {
    ($($g: tt), *) => {
        #[doc(hidden)]
        pub trait MacroMarker<$(const $g: char = ' ',)*> {
            type DelegateType: ?core::marker::Sized;

            fn delegate_by_ref<'a, Output: 'a>(
                &'a self,
                f: impl core::ops::FnOnce(&'a Self::DelegateType) -> Output,
            ) -> Output;

            fn delegate_by_mut<'a, Output: 'a>(
                &'a mut self,
                f: impl core::ops::FnOnce(&'a mut Self::DelegateType) -> Output,
            ) -> Output;
        }
    };
}

expand_macro_maker!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, AA, AB, AC, AD
);
