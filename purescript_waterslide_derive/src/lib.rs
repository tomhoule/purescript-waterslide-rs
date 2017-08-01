extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;

extern crate purescript_waterslide;

mod purescript;

use purescript::make_purs_type;

#[proc_macro_derive(ToPursType)]
pub fn derive_purstype(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();
    let ast =
        syn::parse_derive_input(&input).expect("Purescript waterslide could not parse input type");

    let name = &ast.ident;
    let generics = &ast.generics;

    let to_purs_impl = match make_purs_type(&ast) {
        Ok(generated_impl) => generated_impl,
        Err(err) => panic!("Could not convert the input to Purescript AST: {:?}", err),
    };

    let expanded = quote! {
        impl#generics ::purescript_waterslide::ToPursType for #name#generics {
            fn to_purs_type() -> ::purescript_waterslide::PursType {
                #to_purs_impl
            }
        }
    };

    expanded.parse().unwrap()
}
