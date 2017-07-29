#[macro_use]
extern crate purescript_bridge_codegen;
extern crate purescript_bridge;

use purescript_bridge::*;

#[test]
fn plain_old_struct() {
    #[derive(ToPursType)]
    struct Plain {
        age: i32,
        name: String,
    }

    assert_eq!(
        Plain::to_purs_type(),
        // "data Plain = Plain { age :: i32, name :: String }",
        PursType::Struct(Constructor::Record(RecordConstructor {
            import: None,
            name: "Plain".to_string(),
            arguments: vec![
                ("age".to_string(), PursType::Leaf(Import { type_module: "PRIM" }, "Int".to_string())),
                ("name".to_string(), PursType::Leaf(Import { type_module: "PRIM" }, "String".to_string())),
            ],
        }))
    )
}
