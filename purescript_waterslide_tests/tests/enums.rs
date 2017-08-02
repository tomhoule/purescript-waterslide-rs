#[macro_use]
extern crate purescript_waterslide_derive;
extern crate purescript_waterslide;

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
            "GoodBoy".to_string(),
            vec![
                Constructor::Seq(SeqConstructor {
                    import: None,
                    name: "Doggo".to_string(),
                    arguments: vec![],
                }),
                Constructor::Seq(SeqConstructor {
                    import: None,
                    name: "Pupper".to_string(),
                    arguments: vec![],
                }),
                Constructor::Seq(SeqConstructor {
                    import: None,
                    name: "Shibe".to_string(),
                    arguments: vec![],
                }),
            ]
        )
    );

    assert_eq!(
        &format!("{}", GoodBoy::to_purs_type()),
        "Doggo | Pupper | Shibe"
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
        "Pie Topping | IceCream (Maybe Topping)"
    )
}

#[test]
fn enum_with_tuples() {
    #[derive(ToPursType)]
    enum Dessert { Pie((u8, u8)), Yoghurt((String, u8)) }

    assert_derives_to!(Dessert, "Pie (Tuple Int Int) | Yoghurt (Tuple String Int)")
}
