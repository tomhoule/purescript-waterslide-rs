# purescript-toboggan-rs

## Basic usage

```rust
#[macro_use] extern crate purescript_bridge_codegen;
#[macro_use] extern crate purescript_bridge;

#[derive(ToPursType)]
enum FalafelBasis {
    FavaBean,
    Chickpea,
    Other(Option<String>),
}

#[derive(ToPursType)]
struct Falafel {
    basis: FalafelBasis,
    parsley_percentage: u8,
}

#[derive(ToPursType)]
struct Meal {
    falafels: Vec<Falafel>,
    with_salad: bool,
}

#[test]
fn guten_appetit() {
    let module = purs_module!("Data.Falafel".to_string() : Falafel, FalafelBasis, Meal);
    let expected = "module Data.Falafel where

import Data.Array (
Array
)
import Data.Maybe (
Maybe
)

data Falafel = Falafel { basis :: FalafelBasis, parsley_percentage :: Int }

data FalafelBasis = FavaBean | Chickpea | Other (Maybe String)

data Meal = Meal { falafels :: Array Falafel, with_salad :: bool }
";
    assert_eq!(&format!("{}", &module), expected);
}

```

For running code, take a look at the `examples` directory.

## Features

- Struct and enum definitions that contain structs and enums
- Whole module generation with imports
- You can define custom representations by manually implementing `ToPursType` (unstable interface, will probably change a lot)

### Roadmap

Things I want to add in the coming weeks (in no particular order):

- Parametric types (e.g. `Paginated<T>`, or `Option<T>`)
- Tuple structs
- Custom auto-importing via container attributes
- End to end tests to ensure JSON representation are compatible between
  serde_json and argonaut
- Move the default `ToPursType` implementations to an opt-out feature
- Better documentation and examples

## Contributing

That would be awesome. There are no particular guidelines for pull requests
(maybe in the future). We adhere to the (Rust Code of Conduct)[].

## License

apache + mit blurb here
