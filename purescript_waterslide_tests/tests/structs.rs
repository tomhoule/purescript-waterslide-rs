#![allow(dead_code)]

#[macro_use]
extern crate purescript_waterslide_derive;
extern crate purescript_waterslide;
extern crate void;

use void::Void;
use purescript_waterslide::*;

macro_rules! assert_derives_to {
    ($rust_type:ty, $ps_type:expr) => {
        assert_eq!(
            &format!("{}", <$rust_type as AsPursType>::as_purs_type()),
            $ps_type
        )
    }
}

#[test]
fn plain_old_struct() {
    #[derive(AsPursType)]
    struct Plain {
        age: i32,
        name: String,
    }

    assert_eq!(
        Plain::as_purs_type(),
        PursType::Struct(
            PursConstructor {
                module: None,
                name: "Plain".to_string(),
                parameters: vec![],
            },
            vec![
                (
                    "age".to_string(),
                    PursConstructor {
                        name: "Int".to_string(),
                        module: Some("PRIM".to_string()),
                        parameters: vec![],
                    },
                ),
                (
                    "name".to_string(),
                    PursConstructor {
                        name: "String".to_string(),
                        module: Some("PRIM".to_string()),
                        parameters: vec![],
                    },
                ),
            ]
        )
    );

    assert_eq!(
        &format!("{}", Plain::as_purs_type()),
        "data Plain = Plain { age :: Int, name :: String }"
    );
}

#[test]
fn struct_with_option() {
    #[derive(AsPursType)]
    struct Anonymous {
        age: i32,
        name: Option<String>,
    }

    assert_eq!(
        &format!("{}", Anonymous::as_purs_type()),
        "data Anonymous = Anonymous { age :: Int, name :: Maybe String }"
    );
}

#[test]
fn struct_with_enum() {
    #[derive(AsPursType)]
    enum Color {
        Yellow(bool),
        Purple(String),
    }

    #[derive(AsPursType)]
    struct Anonymous {
        age: i32,
        name: Color,
    }

    assert_eq!(
        &format!("{}", Anonymous::as_purs_type()),
        "data Anonymous = Anonymous { age :: Int, name :: Color }"
    );
}

#[test]
fn newtype_struct() {
    #[derive(AsPursType)]
    struct Email(String);

    assert_eq!(
        &format!("{}", Email::as_purs_type()),
        "data Email = Email String"
    );
}

#[test]
fn tuple_struct() {
    #[derive(AsPursType)]
    struct PersonName(String, String);

    assert_eq!(
        &format!("{}", PersonName::as_purs_type()),
        "data PersonName = PersonName String String"
    );
}

#[test]
fn tuple_struct_with_modifiers() {
    #[derive(AsPursType)]
    struct Node {
        no: u8,
    }

    #[derive(AsPursType)]
    struct Schema(pub &'static [Node]);

    assert_eq!(
        &format!("{}", Schema::as_purs_type()),
        "data Schema = Schema (Array Node)"
    );
}

#[test]
fn struct_with_tuple_fields() {
    #[derive(AsPursType)]
    struct Cow {
        sides: (u8, u8),
        milk: bool,
    }

    assert_derives_to!(
        Cow,
        "data Cow = Cow { sides :: Tuple Int Int, milk :: Boolean }"
    );
}

#[test]
fn unit_struct() {
    #[derive(AsPursType)]
    struct AllRight;

    assert_derives_to!(AllRight, "data AllRight = AllRight");
}

#[test]
fn simple_generic_struct() {
    #[derive(AsPursType)]
    struct Paginated<T> {
        page: u32,
        data: T,
    }

    assert_derives_to!(Paginated<Void>, "data Paginated t = Paginated { page :: Int, data :: t }")
}

#[test]
fn simple_generic_tuple_struct() {
    #[derive(AsPursType)]
    struct Validated<T>(T);

    assert_derives_to!(Validated<Void>, "data Validated t = Validated t")
}
