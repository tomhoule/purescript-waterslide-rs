//! This crate defines the types necessary to translate Rust data types to Purescript. Most users
//! will only need to derive `AsPursType` with the `purescript_waterslide_derive` crate and use
//! the `purs_module!` macro to generate modules.
#![deny(missing_docs)]
#![deny(warnings)]

mod default_implementations;
mod purs_constructor;
mod purs_module;
mod purs_type;

pub use purs_constructor::*;
pub use purs_type::*;
pub use purs_module::*;
