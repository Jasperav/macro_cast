extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree, Ident};
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, Expr, DeriveInput, Type, parse_str};

#[proc_macro_hack]
pub fn add_cast_m(input: TokenStream) -> TokenStream {
    let mut some_ident = None;

    for i in input {
        match i {
            TokenTree::Ident(i) => some_ident = Some(i),
            _ => panic!()
        }
    }

    let some_ident = some_ident.unwrap();
    let casted = format!("{} as i32", some_ident);
    let x: syn::ExprCast = parse_str(&casted).unwrap();

    let q = quote! {
        #x
    };

    q.into()
}