#![deny(warnings)]

extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;

extern crate purescript_waterslide;

mod purescript;
mod generics;

use quote::Tokens;
use purescript::{make_purs_constructor_impl, make_purs_type};

#[proc_macro_derive(AsPursType)]
pub fn derive_purstype(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();
    let ast =
        syn::parse_derive_input(&input).expect("Purescript waterslide could not parse input type");

    let name = &ast.ident;
    let generics = generics::shift_generics(&ast);
    let placeholder_generics: Vec<Tokens> = ast.generics
        .ty_params
        .iter()
        .map(generics::make_dummy_generic)
        .collect();
    let placeholder_generics_clone = placeholder_generics.clone();

    let as_purs_constructor_impl = match make_purs_constructor_impl(&ast) {
        Ok(generated_impl) => generated_impl,
        Err(err) => panic!(
            "Could not convert the input to Purescript type constructor: {:?}",
            err
        ),
    };

    let as_purs_impl = match make_purs_type(&ast) {
        Ok(generated_impl) => generated_impl,
        Err(err) => panic!("Could not convert the input to Purescript AST: {:?}", err),
    };

    let expanded = quote! {
        impl#generics ::purescript_waterslide::AsPursConstructor for #name#generics {
            fn as_purs_constructor() -> ::purescript_waterslide::PursConstructor {
                #( #placeholder_generics )*

                #as_purs_constructor_impl
            }
        }

        impl#generics ::purescript_waterslide::AsPursType for #name#generics {
            fn as_purs_type() -> ::purescript_waterslide::PursType {
                #( #placeholder_generics_clone )*

                #as_purs_impl
            }
        }
    };

    expanded.parse().unwrap()
}
