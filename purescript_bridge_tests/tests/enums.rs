#[macro_use]
extern crate purescript_bridge_codegen;
extern crate purescript_bridge;

use purescript_bridge::*;

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
        PursType::Enum("GoodBoy".to_string(), vec![
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
                       })
        ]));

    assert_eq!(
        &format!("{}", GoodBoy::to_purs_type()),
        "Doggo | Pupper | Shibe | "
    )
}
