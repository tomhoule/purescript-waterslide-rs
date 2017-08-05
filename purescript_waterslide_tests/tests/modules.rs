#![allow(dead_code)]

#[macro_use]
extern crate purescript_waterslide_derive;
extern crate purescript_waterslide;
extern crate void;

use void::*;
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

#[test]
fn module_with_generics() {
    #[derive(ToPursType)]
    enum Page<T> {
        NonEmpty(Vec<T>),
        OOB,
    }

    #[derive(ToPursType)]
    struct Paginated<T, META> {
        page_num: u32,
        contents: Page<T>,
        metadata: META,
    }

    #[derive(ToPursType)]
    struct SomethingElse<T>(T);

    #[derive(ToPursType)]
    struct Proxy;

    let module = purs_module!("Pagination".to_string() ;
                              Paginated<Void, Void>,
                              Page<Void>,
                              SomethingElse<Void>);

    assert_eq!(
        &format!("{}", &module),
        "module Pagination where

import Data.Generic (
class Generic
)

data Paginated t meta = Paginated { page_num :: Int, contents :: Page t, metadata :: meta }

derive instance genericPaginated :: Generic Paginated

data Page t = NonEmpty (Array t) | OOB

derive instance genericPage :: Generic Page

data SomethingElse t = SomethingElse t

derive instance genericSomethingElse :: Generic SomethingElse

"
    );
}
