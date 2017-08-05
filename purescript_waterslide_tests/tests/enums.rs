#[macro_use]
extern crate purescript_waterslide_derive;
extern crate purescript_waterslide;
extern crate void;

use purescript_waterslide::*;
use void::Void;

macro_rules! assert_derives_to {
    ($rust_type:ty, $ps_type:expr) => {
        assert_eq!(
            &format!("{}", <$rust_type as ToPursType>::to_purs_type()),
            $ps_type
        )
    }
}

#[test]
fn plain_old_enum() {
    #[derive(ToPursType)]
    enum GoodBoy {
        Doggo,
        Pupper,
        Shibe,
    }

    assert_eq!(
        GoodBoy::to_purs_type(),
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
        &format!("{}", GoodBoy::to_purs_type()),
        "data GoodBoy = Doggo | Pupper | Shibe"
    )
}

#[test]
fn enum_with_struct_and_option() {
    #[derive(ToPursType)]
    struct Topping {
        ingredient: String,
    };

    #[derive(ToPursType)]
    enum Dessert {
        Pie(Topping),
        IceCream(Option<Topping>),
    }

    assert_eq!(
        &format!("{}", Dessert::to_purs_type()),
        "data Dessert = Pie Topping | IceCream (Maybe Topping)"
    )
}

#[test]
fn enum_with_tuples() {
    #[derive(ToPursType)]
    enum Dessert { Pie((u8, u8)), Yoghurt((String, u8)) }

    assert_derives_to!(Dessert, "data Dessert = Pie (Tuple Int Int) | Yoghurt (Tuple String Int)")
}

#[test]
fn recursive_enum() {
    #[derive(ToPursType)]
    enum Node {
        Branch(Box<Node>),
        Leaf(i32)
    }

    assert_eq!(
        &format!("{}", Node::to_purs_type()),
        "data Node = Branch Node | Leaf Int"
    );
}

#[test]
fn mutually_recursive_types() {
    #[derive(ToPursType)]
    struct NumberedNode {
        number: f32,
        node: &'static Node,
    }

    #[derive(ToPursType)]
    enum Node {
        Branch(NumberedNode),
        Leaf(i32)
    }

    assert_eq!(
        &format!("{}", Node::to_purs_type()),
        "data Node = Branch NumberedNode | Leaf Int"
    );
}

#[test]
fn simple_generic_enum() {
    #[derive(ToPursType)]
    enum Choice<L, R> {
        Left(L),
        Right(R),
    }

    assert_eq!(
        &format!("{}", Choice::<Void, Void>::to_purs_type()),
        "data Choice l r = Left l | Right r"
    );
}
