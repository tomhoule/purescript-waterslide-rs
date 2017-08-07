extern crate chrono;

use purs_constructor::{AsPursConstructor, PursConstructor};

impl<T> AsPursConstructor for chrono::DateTime<T>
where
    T: chrono::TimeZone,
{
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            module: None,
            name: "String".to_string(),
            parameters: vec![],
        }
    }
}

impl<T> AsPursConstructor for chrono::Date<T>
where
    T: chrono::TimeZone,
{
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            module: None,
            name: "String".to_string(),
            parameters: vec![],
        }
    }
}

impl AsPursConstructor for chrono::naive::NaiveDate {
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            module: None,
            name: "String".to_string(),
            parameters: vec![],
        }
    }
}

impl AsPursConstructor for chrono::naive::NaiveTime {
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            module: None,
            name: "String".to_string(),
            parameters: vec![],
        }
    }
}

impl AsPursConstructor for chrono::naive::NaiveDateTime {
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            module: None,
            name: "String".to_string(),
            parameters: vec![],
        }
    }
}
