extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree, Ident};
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::{parse_macro_input, Expr, DeriveInput, Type};

#[proc_macro_hack]
pub fn query(input: TokenStream) -> TokenStream {
    let mut some_ident = None;

    for i in input {
        match i {
            TokenTree::Ident(i) => some_ident = Some(i),
            _ => panic!()
        }
    }

    let some_ident = some_ident.unwrap();
    let stringified = ident_to_string(&some_ident);
    let new_ty = determine_type(stringified);

    quote! {
        #x as #new_ty
    }
}

fn ident_to_string(ident: &Ident) -> &'static str {
    match ident.to_string().as_str() {
        "i32" => "tinyint",
        _ => panic!()
    }
}

fn determine_type(ident: &str) -> ?? {
    match ident {
        "tinyint" -> i64
        _ => panic!()
    }
}