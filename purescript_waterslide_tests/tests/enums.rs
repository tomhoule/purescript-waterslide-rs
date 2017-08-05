#![allow(dead_code)]

#[macro_use]
extern crate purescript_waterslide_derive;
extern crate purescript_waterslide;
extern crate void;

use purescript_waterslide::*;
use void::Void;

macro_rules! assert_derives_to {
    ($rust_type:ty, $ps_type:expr) => {
        assert_eq!(
            &format!("{}", <$rust_type as AsPursType>::as_purs_type()),
            $ps_type
        )
    }
}

#[test]
fn plain_old_enum() {
    #[derive(AsPursType)]
    enum GoodBoy {
        Doggo,
        Pupper,
        Shibe,
    }

    assert_eq!(
        GoodBoy::as_purs_type(),
        // data GoodBoy = Doggo | Pupper | Shibe
        PursType::Enum(
            PursConstructor {
                name: "GoodBoy".to_string(),
                module: None,
                parameters: vec![],
            },
            vec![
                PursConstructor {
                    module: None,
                    name: "Doggo".to_string(),
                    parameters: vec![],
                },
                PursConstructor {
                    module: None,
                    name: "Pupper".to_string(),
                    parameters: vec![],
                },
                PursConstructor {
                    module: None,
                    name: "Shibe".to_string(),
                    parameters: vec![],
                },
            ]
        )
    );

    assert_eq!(
        &format!("{}", GoodBoy::as_purs_type()),
        "data GoodBoy = Doggo | Pupper | Shibe"
    )
}

#[test]
fn enum_with_struct_and_option() {
    #[derive(AsPursType)]
    struct Topping {
        ingredient: String,
    };

    #[derive(AsPursType)]
    enum Dessert {
        Pie(Topping),
        IceCream(Option<Topping>),
    }

    assert_eq!(
        &format!("{}", Dessert::as_purs_type()),
        "data Dessert = Pie Topping | IceCream (Maybe Topping)"
    )
}

#[test]
fn enum_with_tuples() {
    #[derive(AsPursType)]
    enum Dessert {
        Pie((u8, u8)),
        Yoghurt((String, u8)),
    }

    assert_derives_to!(
        Dessert,
        "data Dessert = Pie (Tuple Int Int) | Yoghurt (Tuple String Int)"
    )
}

#[test]
fn recursive_enum() {
    #[derive(AsPursType)]
    enum Node {
        Branch(Box<Node>),
        Leaf(i32),
    }

    assert_eq!(
        &format!("{}", Node::as_purs_type()),
        "data Node = Branch Node | Leaf Int"
    );
}

#[test]
fn mutually_recursive_types() {
    #[derive(AsPursType)]
    struct NumberedNode {
        number: f32,
        node: &'static Node,
    }

    #[derive(AsPursType)]
    enum Node {
        Branch(NumberedNode),
        Leaf(i32),
    }

    assert_eq!(
        &format!("{}", Node::as_purs_type()),
        "data Node = Branch NumberedNode | Leaf Int"
    );
}

#[test]
fn simple_generic_enum() {
    #[derive(AsPursType)]
    enum Choice<L, R> {
        Left(L),
        Right(R),
    }

    assert_eq!(
        &format!("{}", Choice::<Void, Void>::as_purs_type()),
        "data Choice l r = Left l | Right r"
    );
}
