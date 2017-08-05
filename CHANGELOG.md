# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## Unreleased

### Added

* Generic data types support

## [0.2.1] - 2017-08-03

### Fixed

* Fixed stack overflow when deriving recursive types
* The Purescript AST was significantly simplified
* Added default ToPursType implementations for `Box<T>` and `&T`

## [0.2.0] - 2017-08-01

### Added

* Tuple struct support
* `ToPursType` implementations for `&[T]` and `&str`.

## [0.1.0] - 2017-07-30

* Initial release
