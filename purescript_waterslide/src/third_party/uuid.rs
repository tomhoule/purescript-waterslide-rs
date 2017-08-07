extern crate uuid;

use purs_constructor::{AsPursConstructor, PursConstructor};

impl AsPursConstructor for uuid::Uuid {
    fn as_purs_constructor() -> PursConstructor {
        PursConstructor {
            module: None,
            name: "String".to_string(),
            parameters: vec![],
        }
    }
}
