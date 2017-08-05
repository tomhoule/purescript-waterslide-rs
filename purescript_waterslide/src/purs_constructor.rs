use std::fmt::{Display, Formatter};

pub trait ToPursConstructor {
    fn to_purs_constructor() -> PursConstructor;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PursConstructor {
    pub module: Option<String>,
    pub name: String,
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
