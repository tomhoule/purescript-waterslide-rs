use syn;
use syn::{Body, VariantData};
use syn::{DeriveInput};
use quote::{Tokens, ToTokens};
use proc_macro2;
use proc_macro2::{Delimiter, TokenTree, TokenStream, TokenNode, Term, Literal};
use std::fmt::{Formatter, Display};
use purescript_bridge::*;
use std::iter::FromIterator;

struct VariantName<'a>(&'a syn::Variant);

impl<'a> ToTokens for VariantName<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let name = format!("{}", &self.0.ident);
        tokens.append(quote!(#name.to_string()))
    }
}

struct VariantArguments<'a>(&'a syn::Variant);

impl<'a> ToTokens for VariantArguments<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        if let VariantData::Tuple(ref fields) = self.0.data {
            let tys = fields.iter().map(|f| &f.ty);
            tokens.append(quote!{
                vec![
                    #( <#tys as ToPursType>::to_purs_type()  ),*
                ]
            })
        } else {
            tokens.append(quote!{ vec![] })
        }
    }
}


struct RecordField<'a>(&'a syn::Field);

impl<'a> ToTokens for RecordField<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let name = self.0.ident.clone().map(|id| format!("{}", id)).unwrap_or("_unknown".to_string());
        let ty = &self.0.ty;
        tokens.append(quote!{
            (#name.to_string(), <#ty as ::purescript_bridge::ToPursType>::to_purs_type())
        })
    }
}

// Implements ToPursType recursively by injecting `to_purs_type` calls on every field/variant
enum ToPursTypeImpl {
    Enum(EnumImpl),
    Struct(StructImpl),
}

struct StructImpl {
    name: String,
    constructor: String,
}

struct EnumImpl {
    name: String,
    constructors: Vec<String>,
}

pub fn make_purs_type(source: &DeriveInput) -> Result<Tokens, ()> {
    let name = format!("{}", &source.ident);
    match source.body {
        Body::Enum(ref variants) => {
            let variant_names = variants.iter().map(VariantName);
            let variant_arguments = variants.iter().map(VariantArguments);
            Ok(quote! {
                PursType::Enum(
                    #name.to_string(),
                    vec![
                        #( ::purescript_bridge::Constructor::Seq(
                                ::purescript_bridge::SeqConstructor {
                                    import: None,
                                    name: #variant_names,
                                    arguments: #variant_arguments,
                                })
                        ),*
                    ],
                )
            })
        },
        Body::Struct(VariantData::Struct(ref fields)) => {
            let purs_record_fields = fields.into_iter().map(RecordField);
            Ok(quote! {
                PursType::Struct(Constructor::Record(RecordConstructor {
                    import: None,
                    name: #name.to_string(),
                    arguments: vec![
                        #( #purs_record_fields ),*
                    ],
                }))
            })
        },
        _ => Err(())
    }

    // Ok(PursTypeImpl { ident: source.ident.clone(), body: source.body.clone().into() })
}
