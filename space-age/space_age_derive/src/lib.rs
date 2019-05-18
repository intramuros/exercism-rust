#![recursion_limit = "128"]
extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Planet)]
pub fn planet_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl Planet for #name {
            fn years_during(d: &Duration) -> f64 {
                d.val / #name::period()
            }
        }
    };
    gen.into()
}
