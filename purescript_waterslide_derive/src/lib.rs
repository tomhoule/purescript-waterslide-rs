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

#[proc_macro_derive(ToPursType)]
pub fn derive_purstype(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = input.to_string();
    let ast =
        syn::parse_derive_input(&input).expect("Purescript waterslide could not parse input type");

    let name = &ast.ident;
    let generics = generics::shift_generics(&ast);
    let placeholder_generics: Vec<Tokens> = ast.generics.ty_params.iter().map(|param| {
        let type_ident = &param.ident;
        let type_name = format!("{}", &param.ident);
        quote!{
            struct #type_ident;

            impl ToPursConstructor for #type_ident {
                fn to_purs_constructor() -> PursConstructor {
                    PursConstructor {
                        module: None,
                        name: #type_name.to_string(),
                        parameters: vec![],
                    }
                }
            }

        }
    }).collect();
    let placeholder_generics_clone = placeholder_generics.clone();

    let to_purs_constructor_impl = match make_purs_constructor_impl(&ast) {
        Ok(generated_impl) => generated_impl,
        Err(err) => panic!("Could not convert the input to Purescript type constructor: {:?}", err),
    };

    let to_purs_impl = match make_purs_type(&ast) {
        Ok(generated_impl) => generated_impl,
        Err(err) => panic!("Could not convert the input to Purescript AST: {:?}", err),
    };

    let expanded = quote! {
        impl#generics ::purescript_waterslide::purs_constructor::ToPursConstructor for #name#generics {
            fn to_purs_constructor() -> ::purescript_waterslide::purs_constructor::PursConstructor {
                #( #placeholder_generics )*

                #to_purs_constructor_impl
            }
        }

        impl#generics ::purescript_waterslide::ToPursType for #name#generics {
            fn to_purs_type() -> ::purescript_waterslide::PursType {
                #( #placeholder_generics_clone )*

                #to_purs_impl
            }
        }
    };

    expanded.parse().unwrap()
}
