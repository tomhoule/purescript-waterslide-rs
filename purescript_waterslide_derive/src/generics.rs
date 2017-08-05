use syn::*;
use quote::Tokens;

// This function is responsible for altering the type parameter names so they are not in scope in
// the impl block.
pub fn shift_generics(ast: &DeriveInput) -> Generics {
    let shifted_type_parameters: Vec<TyParam> = ast.generics
        .ty_params
        .iter()
        .map(|param| {
            let new_name = format!("{}_", param.ident);
            TyParam {
                attrs: param.attrs.clone(),
                ident: new_name.into(),
                bounds: param.bounds.clone(),
                default: param.default.clone(),
            }
        })
        .collect();

    Generics {
        lifetimes: ast.generics.lifetimes.clone(),
        ty_params: shifted_type_parameters,
        where_clause: ast.generics.where_clause.clone(),
    }
}

/// Generates dummy implementations for the type parameters inside the data type declaration body
///
/// It goes from (generics already shifted with shift_generics):
///
/// struct Paginated<T_> {
///  page: u32,
///  items: Vec<T>
/// }
///
/// to:
///
/// struct T;
///
/// impl AsPursConstructor for T { ...  }
///
pub fn make_dummy_generic(param: &TyParam) -> Tokens {
    let type_ident = &param.ident;
    let type_name = format!("{}", &param.ident).to_lowercase();
    quote!{
        struct #type_ident;

        impl AsPursConstructor for #type_ident {
            fn as_purs_constructor() -> PursConstructor {
                PursConstructor {
                    module: None,
                    name: #type_name.to_string(),
                    parameters: vec![],
                }
            }
        }
    }
}
