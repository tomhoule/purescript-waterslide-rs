use purs_constructor::*;

impl<T: AsPursConstructor> AsPursConstructor for Vec<T> {
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            name: "Array".to_string(),
            module: None,
            parameters: vec![<T as AsPursConstructor>::as_purs_constructor()],
        }
    }
}

impl<'a, T: AsPursConstructor> AsPursConstructor for &'a [T] {
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            name: "Array".to_string(),
            module: None,
            parameters: vec![<T as AsPursConstructor>::as_purs_constructor()],
        }
    }
}

impl<T: AsPursConstructor> AsPursConstructor for Option<T> {
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            name: "Maybe".to_string(),
            module: Some("Data.Maybe".to_string()),
            parameters: vec![<T as AsPursConstructor>::as_purs_constructor()],
        }
    }
}

impl<'a> AsPursConstructor for &'a str {
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            name: "String".to_string(),
            module: None,
            parameters: vec![],
        }
    }
}

impl<T, U> AsPursConstructor for (T, U)
where
    T: AsPursConstructor,
    U: AsPursConstructor,
{
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            name: "Tuple".to_string(),
            module: Some("Data.Tuple".to_string()),
            parameters: vec![
                <T as AsPursConstructor>::as_purs_constructor(),
                <U as AsPursConstructor>::as_purs_constructor(),
            ],
        }
    }
}

impl AsPursConstructor for () {
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            module: Some("Prelude".to_string()),
            name: "Tuple".to_string(),
            parameters: vec![],
        }
    }
}

impl<T: AsPursConstructor> AsPursConstructor for Box<T> {
    fn as_purs_constructor() -> PursConstructor {
        T::as_purs_constructor()
    }
}

impl<'a, T: AsPursConstructor> AsPursConstructor for &'a T {
    fn as_purs_constructor() -> PursConstructor {
        T::as_purs_constructor()
    }
}

macro_rules! purs_primitive_impl {
    ($rust_type:ty, $purs_type:expr, $import:expr) => {
        impl AsPursConstructor for $rust_type {
            fn as_purs_constructor() -> PursConstructor {
                PursConstructor {
                    module: Some($import.to_string()),
                    name: $purs_type.to_string(),
                    parameters: vec![],
                }
            }
        }
    }
}

const PRIM: &'static str = "PRIM";

purs_primitive_impl!(bool, "Boolean", PRIM);

purs_primitive_impl!(i8, "Int", PRIM);
purs_primitive_impl!(i16, "Int", PRIM);
purs_primitive_impl!(i32, "Int", PRIM);
purs_primitive_impl!(i64, "Int", PRIM);
purs_primitive_impl!(isize, "Int", PRIM);

purs_primitive_impl!(u8, "Int", PRIM);
purs_primitive_impl!(u16, "Int", PRIM);
purs_primitive_impl!(u32, "Int", PRIM);
purs_primitive_impl!(u64, "Int", PRIM);
purs_primitive_impl!(usize, "Int", PRIM);

purs_primitive_impl!(f32, "Number", PRIM);
purs_primitive_impl!(f64, "Number", PRIM);

purs_primitive_impl!(String, "String", PRIM);
