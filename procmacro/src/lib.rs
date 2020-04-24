extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::{TokenStream, Span, TokenTree};
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse_quote;
use syn::parse_macro_input;

#[proc_macro_hack]
pub fn add_casts(input: TokenStream) -> TokenStream {
    let mut index = -1;
    let mut text = String::new();
    let mut idents: Vec<syn::Ident> = vec![];

    for i in input {
        index += 1;

        if index % 2 != 0 {
            // Each element should be separated by a comma
            match i {
                TokenTree::Punct(l) => assert_eq!(",", l.to_string()),
                _ => panic!("Expected punct, got {:#?}", i)
            }
            continue;
        }

        if index == 0 {
            // This is the text
            match i {
                TokenTree::Literal(l) => text = l.to_string(),
                _ => panic!("Expected literal, got {:#?}", i)
            }
        } else {
            match i {
                TokenTree::Ident(i) => idents.push(syn::parse_str(&i.to_string()).unwrap()),
                _ => panic!("Expected ident, got {:#?}", i)
            }
        }
    }

    let q = quote! {
        (#text, #(#idents as i32),*)
    };

    q.into()
}