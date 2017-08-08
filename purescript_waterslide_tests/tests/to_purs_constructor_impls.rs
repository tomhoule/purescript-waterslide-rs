extern crate purescript_waterslide;
extern crate chrono;
extern crate uuid;

use chrono::*;

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

#[test]
fn uuid_translates_as_expected() {
    assert_eq!(&format!("{}", uuid::Uuid::as_purs_constructor()), "String");
}

#[test]
fn chrono_datetime_translates_as_expected() {
    assert_eq!(
        &format!("{}", DateTime::<Utc>::as_purs_constructor()),
        "String"
    );
}

#[test]
fn chrono_date_translates_as_expected() {
    assert_eq!(&format!("{}", Date::<Utc>::as_purs_constructor()), "String");
}

#[test]
fn chrono_naive_datetime_translates_as_expected() {
    assert_eq!(
        &format!("{}", NaiveDateTime::as_purs_constructor()),
        "String"
    );
}

#[test]
fn chrono_naive_date_translates_as_expected() {
    assert_eq!(&format!("{}", NaiveDate::as_purs_constructor()), "String");
}

#[test]
fn chrono_naive_time_translates_as_expected() {
    assert_eq!(&format!("{}", NaiveTime::as_purs_constructor()), "String");
}
