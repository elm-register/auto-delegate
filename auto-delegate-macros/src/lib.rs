#![feature(iter_intersperse)]
#![feature(core_intrinsics)]
#![feature(proc_macro_span)]
#![feature(iterator_try_collect)]

use proc_macro::TokenStream;

use crate::delegate_struct::expand_delegate;
use crate::delegate_trait::expand_delegate_trait;

mod delegate_struct;
mod delegate_trait;
mod ident;
mod macro_marker;
mod span;
mod syn;
mod trait_item;
mod attribute;

#[proc_macro_attribute]
pub fn delegate(attr: TokenStream, input: TokenStream) -> TokenStream {
    let output = expand_delegate_trait(attr, input.clone());
    expand_join(input, output)
}


#[proc_macro_derive(Delegate, attributes(to))]
pub fn derive_delegate(input: TokenStream) -> TokenStream {
    expand_delegate(input).into()
}


fn expand_join(input: TokenStream, output: proc_macro2::TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let expand = quote::quote! {
        #input
        #output
    };

    expand.into()
}
