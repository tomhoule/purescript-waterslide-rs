# purescript-waterslide-rs
![travis badge](https://travis-ci.org/tomhoule/purescript-waterslide-rs.svg?branch=master)
![crates.io badge](https://img.shields.io/crates/v/purescript_waterslide.svg)

![logo](purescript_waterslide.jpg)

Wouldn't it be nice if you could share your data type definitions between your
Rust backend and your Purescript app? Now you can!

Waterslide generates Purescript type definitions from Rust structs and enums.
It works on stable Rust.

The library is still young but the core features are working. The idea is to
release early, release often and gather feedback to get to a 1.0 version soon.

## Basic usage

- Derive Purescript representations by annotating your structs and enums with
  `#[derive(AsPursType)]` or by manually implementing `AsPursType` if you have
  a custom serialization scheme.
- Define a module with the `purs_module!` macro (e.g.
  `purs_module!("Data.Dogs".to_string() ; Dachsund, ChowChow, Mutt<Void, Void>)`).
- Print the module to stdout or directly to a file using `PursModule`'s
  `Display` impl.

You might want to put the module generation code in a separate binary, which is easy to do with Cargo.

**Important**: on the Purescript side, you want to use the
`Data.Argonaut.Generic.Aeson` codec to encode and decode the JSON types.

**Important\***: on the Rust side, your enums have to be annotated with
`#[serde(tag = "tag", content = "contents")]`

These restrictions will be lifted in the future by the development of an
Argonaut codec that mirrors `serde_json`'s defaults.

For running code, take a look at the [basic example](examples/basic). The tests
also provide a lot of usage examples, notably for generic types.

## Features

- Struct and enum definitions, including tuple structs.
- Default implementations for primitive types and standard library collections (`Vec`...)
- Support for generic types (e.g. `Alternative<T, U>`, `Paginated<T>`...)
- Whole module generation with imports
- You can define custom representations by manually implementing `AsPursType` (unstable interface)

### Roadmap

Things I want to add in the coming weeks (in no particular order):

- More end to end tests to ensure JSON representation are compatible between
  serde_json and argonaut.
- Better documentation and examples

## Acknowledgments

The idea of this library came from Haskell's
[purescript-bridge](https://github.com/eskimor/purescript-bridge) package.

## Contributing

That would be awesome! There are no particular guidelines for pull requests
(maybe in the future). We adhere to the [Rust Code of
Conduct](https://www.rust-lang.org/en-US/conduct.html).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
        http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
        http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
