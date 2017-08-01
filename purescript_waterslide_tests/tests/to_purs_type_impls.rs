#[macro_use]
extern crate purescript_waterslide_derive;
extern crate purescript_waterslide;

use purescript_waterslide::ToPursType;

#[test]
fn str_derives_as_expected() {
    assert_eq!(&format!("{}", <&str>::to_purs_type()), "String");

}

#[test]
fn slices_derive_as_expected() {
    assert_eq!(&format!("{}", <&[u8]>::to_purs_type()), "Array Int");

}

#[test]
fn vecs_derive_as_expected() {
    assert_eq!(&format!("{}", <Vec<u8>>::to_purs_type()), "Array Int");

}