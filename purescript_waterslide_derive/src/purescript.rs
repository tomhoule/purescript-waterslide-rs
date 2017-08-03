use syn;
use syn::{Body, VariantData, Ident};
use syn::DeriveInput;
use quote::{ToTokens, Tokens};
use proc_macro2;
use proc_macro2::{Delimiter, Literal, Term, TokenNode, TokenStream, TokenTree};
use std::fmt::{Display, Formatter};
use purescript_waterslide::*;
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
                    #( <#tys as ::purescript_waterslide::ToPursConstructor>::to_purs_constructor()  ),*
                ]
            })
        } else {
            tokens.append(quote!{ vec![] })
        }
    }
}

struct TupleField<'a>(&'a syn::Field);

impl<'a> ToTokens for TupleField<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let ty = &self.0.ty;
        tokens.append(quote!{ <#ty as ::purescript_waterslide::ToPursConstructor>::to_purs_constructor() })
    }
}

struct RecordField<'a>(&'a syn::Field);

impl<'a> ToTokens for RecordField<'a> {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let name = self.0
            .ident
            .clone()
            .map(|id| format!("{}", id))
            .unwrap_or("_unknown".to_string());
        let ty = &self.0.ty;
        tokens.append(quote!{
            (#name.to_string(), <#ty as ::purescript_waterslide::ToPursConstructor>::to_purs_constructor())
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

pub fn make_purs_type(source: &DeriveInput) -> Result<Tokens, String> {
    let name = format!("{}", &source.ident);
    match source.body {
        Body::Enum(ref variants) => {
            let variant_names = variants.iter().map(VariantName);
            let variant_arguments = variants.iter().map(VariantArguments);
            Ok(quote! {
                ::purescript_waterslide::PursType::Enum(
                    #name.to_string(),
                    vec![
                        #( ::purescript_waterslide::Constructor::Seq(
                                ::purescript_waterslide::SeqConstructor {
                                    import: None,
                                    name: #variant_names,
                                    arguments: #variant_arguments,
                                })
                        ),*
                    ],
                )
            })
        }
        Body::Struct(VariantData::Struct(ref fields)) => {
            let purs_record_fields = fields.into_iter().map(RecordField);
            Ok(quote! {
                ::purescript_waterslide::PursType::Struct(
                    ::purescript_waterslide::Constructor::Record(
                        ::purescript_waterslide::RecordConstructor {
                            import: None,
                            name: #name.to_string(),
                            arguments: vec![
                                #( #purs_record_fields ),*
                            ],
                        }
                    )
                )
            })
        },
        Body::Struct(VariantData::Tuple(ref fields)) => {
            let purs_tuple_fields = fields.iter().map(TupleField);
            Ok(quote! {
                ::purescript_waterslide::PursType::Enum(
                    #name.to_string(),
                    vec![
                        ::purescript_waterslide::Constructor::Seq(
                            ::purescript_waterslide::SeqConstructor {
                                import: None,
                                name: #name.to_string(),
                                arguments: vec![
                                    #( #purs_tuple_fields ),*
                                ],
                            }
                        )
                    ],
                )
            })
        },
        Body::Struct(VariantData::Unit) => Err("Unit type".to_string()),
    }
}

pub fn make_purs_constructor_impl(ast: &DeriveInput) -> Result<Tokens, String> {
    let name = format!("{}", &ast.ident);
    let parameters: Vec<Ident> = ast.generics.ty_params.iter().map(|param| param.ident.clone()).collect();
    Ok(quote! {
        ::purescript_waterslide::purs_constructor::PursConstructor {
            name: #name.to_string(),
            module: None,
            parameters: vec![
                #( <#parameters as ::purescript_waterslide::purs_constructor::ToPursConstructor>::to_purs_constructor() ),*
            ],
        }
    })
}
