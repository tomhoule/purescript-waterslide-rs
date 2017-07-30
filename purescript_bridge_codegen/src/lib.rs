extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;

extern crate purescript_bridge;

mod purescript;

use purescript::make_purs_type;

#[proc_macro_derive(ToPursType)]
pub fn derive_purstype(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();
    let ast = syn::parse_derive_input(&input).expect("Purescript bridge could not parse input type");

    let name = &ast.ident;

    let to_purs_impl = make_purs_type(&ast).expect("Could not convert the input to Purescript AST");

    let expanded = quote! {
        impl ::purescript_bridge::ToPursType for #name {
            fn to_purs_type() -> ::purescript_bridge::PursType {
                #to_purs_impl
            }
        }
    };

    expanded.parse().unwrap()
}
