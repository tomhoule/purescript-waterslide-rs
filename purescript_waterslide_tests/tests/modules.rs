#[macro_use]
extern crate purescript_waterslide_codegen;
extern crate purescript_waterslide;

#[macro_use]
use purescript_waterslide::*;

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

import Data.Generic (
class Generic
)

data Currency = Coins | Credits | Abolished

derive instance genericCurrency :: Generic Currency

data Color = Red Int | Green Int | Blue (Array Int)

derive instance genericColor :: Generic Color

data Fruit = Fruit { color :: Color, price :: Int, currency :: Currency }

derive instance genericFruit :: Generic Fruit

"
    );
}
