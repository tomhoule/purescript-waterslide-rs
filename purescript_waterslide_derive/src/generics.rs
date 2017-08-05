use syn::*;

// This function is responsible for altering the type parameter names so they are not in scope in
// the impl block.
pub fn shift_generics(ast: &DeriveInput) -> Generics {
    let shifted_type_parameters: Vec<TyParam> = ast.generics.ty_params.iter()
        .map(|param| {
            let new_name = format!("{}8", param.ident);
            TyParam {
                attrs: param.attrs.clone(),
                ident: new_name.into(),
                bounds: param.bounds.clone(),
                default: param.default.clone(),
            }
        }).collect();

    Generics {
        lifetimes: ast.generics.lifetimes.clone(),
        ty_params: shifted_type_parameters,
        where_clause: ast.generics.where_clause.clone(),
    }
}
