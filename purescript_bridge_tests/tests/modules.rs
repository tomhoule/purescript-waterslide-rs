#[macro_use]
extern crate purescript_bridge_codegen;
extern crate purescript_bridge;

#[macro_use]
use purescript_bridge::*;

#[derive(ToPursType)]
enum Color {
    Red(u8),
    Green(u8),
    Blue(Vec<u8>),
}

#[derive(ToPursType)]
enum Currency {
    Coins,
    Credits,
    Abolished,
}

#[derive(ToPursType)]
struct Fruit {
    color: Color,
    price: u64,
    currency: Currency,
}

#[test]
fn module_format() {
    let module = purs_module!("Fruits".to_string() ; Currency, Color, Fruit);
    assert_eq!(
        &format!("{}", &module),
        "module Fruits where

data Currency = Coins | Credits | Abolished

data Color = Red Int | Green Int | Blue (Array Int)

data Fruit = Fruit { color :: Color, price :: Int, currency :: Currency, }

"
    );
}
