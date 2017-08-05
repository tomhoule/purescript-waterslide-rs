extern crate purescript_waterslide;

use purescript_waterslide::AsPursConstructor;

#[test]
fn str_derives_as_expected() {
    assert_eq!(&format!("{}", <&str>::as_purs_constructor()), "String");

}

#[test]
fn slices_derive_as_expected() {
    assert_eq!(&format!("{}", <&[u8]>::as_purs_constructor()), "Array Int");

}

#[test]
fn vecs_derive_as_expected() {
    assert_eq!(
        &format!("{}", <Vec<u8>>::as_purs_constructor()),
        "Array Int"
    );

}

#[test]
fn boxes_derive_as_expected() {
    assert_eq!(&format!("{}", <Box<u8>>::as_purs_constructor()), "Int");

}
