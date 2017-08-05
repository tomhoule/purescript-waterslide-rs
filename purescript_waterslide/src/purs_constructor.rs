use std::fmt::{Display, Formatter};

/// Produce a `PursConstructor` from a Rust type.
pub trait ToPursConstructor {
    /// Statically produces a `PursConstructor`.
    fn to_purs_constructor() -> PursConstructor;
}

/// Represents a Purescript type name with its parameters and which module it comes from.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PursConstructor {
    /// The Purescript module this type comes from. For example for `Option<T>` this is
    /// `Some("Data.Maybe".to_string())`.
    pub module: Option<String>,
    /// The Purescript name of this type. For `Option<T>` this would be "Maybe".
    pub name: String,
    /// The parameters this type accepts. For `Option<i32>` this would be the PursConstructor for
    /// i32.
    pub parameters: Vec<PursConstructor>,
}

impl Display for PursConstructor {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "{}", &self.name)?;

        for parameter in &self.parameters {
            if parameter.parameters.is_empty() {
                write!(f, " {}", parameter)?;
            } else {
                write!(f, " ({})", parameter)?;
            }
        }

        Ok(())
    }
}
