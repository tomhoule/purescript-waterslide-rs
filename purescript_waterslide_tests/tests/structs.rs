#[macro_use]
extern crate purescript_waterslide_codegen;
extern crate purescript_waterslide;

use purescript_waterslide::*;

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
    );

    assert_eq!(
        &format!("{}", Plain::to_purs_type()),
        "Plain { age :: Int, name :: String }"
    );
}

#[test]
fn struct_with_option() {
    #[derive(ToPursType)]
    struct Anonymous {
        age: i32,
        name: Option<String>,
    }

    assert_eq!(
        &format!("{}", Anonymous::to_purs_type()),
        "Anonymous { age :: Int, name :: (Maybe String) }"
    );
}

#[test]
fn struct_with_enum() {
    #[derive(ToPursType)]
    enum Color { Yellow(bool), Purple(String) }

    #[derive(ToPursType)]
    struct Anonymous {
        age: i32,
        name: Color,
    }

    assert_eq!(
        &format!("{}", Anonymous::to_purs_type()),
        "Anonymous { age :: Int, name :: Color }"
    );
}
