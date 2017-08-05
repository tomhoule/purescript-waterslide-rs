#[macro_use]
extern crate purescript_waterslide_derive;
extern crate purescript_waterslide;
extern crate void;

use void::Void;
use purescript_waterslide::*;

macro_rules! assert_derives_to {
    ($rust_type:ty, $ps_type:expr) => {
        assert_eq!(
            &format!("{}", <$rust_type as ToPursType>::to_purs_type()),
            $ps_type
        )
    }
}

#[test]
fn plain_old_struct() {
    #[derive(ToPursType)]
    struct Plain {
        age: i32,
        name: String,
    }

    assert_eq!(
        Plain::to_purs_type(),
        PursType::Struct(PursConstructor {
            module: None,
            name: "Plain".to_string(),
            parameters: vec![],
        }, vec![
            ("age".to_string(), PursConstructor {
                name: "Int".to_string(),
                module: Some("PRIM".to_string()),
                parameters: vec![],
            }),
            ("name".to_string(), PursConstructor {
                name: "String".to_string(),
                module: Some("PRIM".to_string()),
                parameters: vec![],
            })
        ])
    );

    assert_eq!(
        &format!("{}", Plain::to_purs_type()),
        "data Plain = Plain { age :: Int, name :: String }"
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
        "data Anonymous = Anonymous { age :: Int, name :: Maybe String }"
    );
}

#[test]
fn struct_with_enum() {
    #[derive(ToPursType)]
    enum Color {
        Yellow(bool),
        Purple(String),
    }

    #[derive(ToPursType)]
    struct Anonymous {
        age: i32,
        name: Color,
    }

    assert_eq!(
        &format!("{}", Anonymous::to_purs_type()),
        "data Anonymous = Anonymous { age :: Int, name :: Color }"
    );
}

#[test]
fn newtype_struct() {
    #[derive(ToPursType)]
    struct Email(String);

    assert_eq!(
        &format!("{}", Email::to_purs_type()),
        "data Email = Email String"
    );
}

#[test]
fn tuple_struct() {
    #[derive(ToPursType)]
    struct PersonName(String, String);

    assert_eq!(
        &format!("{}", PersonName::to_purs_type()),
        "data PersonName = PersonName String String"
    );
}

#[test]
fn tuple_struct_with_modifiers() {
    #[derive(ToPursType)]
    struct Node { no: u8 }

    #[derive(ToPursType)]
    struct Schema(pub &'static [Node]);

    assert_eq!(
        &format!("{}", Schema::to_purs_type()),
        "data Schema = Schema (Array Node)"
    );
}

#[test]
fn struct_with_tuple_fields() {
    #[derive(ToPursType)]
    struct Cow { sides: (u8, u8), milk: bool }

    assert_derives_to!(Cow, "data Cow = Cow { sides :: Tuple Int Int, milk :: Boolean }");
}

#[test]
fn unit_struct() {
    #[derive(ToPursType)]
    struct AllRight;

    assert_derives_to!(AllRight, "data AllRight = AllRight");
}

#[test]
fn simple_generic_struct() {
    #[derive(ToPursType)]
    struct Paginated<T> {
        page: u32,
        data: T,
    }

    assert_derives_to!(Paginated<Void>, "data Paginated t = Paginated { page :: Int, data :: t }")
}

#[test]
fn simple_generic_tuple_struct() {
    #[derive(ToPursType)]
    struct Validated<T>(T);

    assert_derives_to!(Validated<Void>, "data Validated t = Validated t")
}
