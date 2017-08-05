use std::fmt::{Display, Formatter};
use purs_constructor::*;

/// The representation for a Purescript data type declaration. The PursType for a Rust struct and
/// enum can be obtained by deriving the ToPursType trait.
#[derive(Clone, Debug, PartialEq)]
pub enum PursType {
    Struct(PursConstructor, Vec<(String, PursConstructor)>),
    TupleStruct(PursConstructor, Vec<PursConstructor>),
    Enum(PursConstructor, Vec<PursConstructor>),
}

impl Display for PursType {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        use PursType::*;

        match *self {
            Struct(ref type_, ref fields) => {
                write!(f, "data {} ", type_.name)?;

                for ref param in type_.parameters.iter() {
                    write!(f, "{} ", param.name)?;
                }

                write!(f, "= {} {{ ", type_.name)?;

                for (idx, &(ref name, ref constructor)) in fields.iter().enumerate() {
                    write!(f, "{} :: {}", name, constructor)?;
                    if idx < (fields.len() - 1) {
                        write!(f, ",")?;
                    }
                    write!(f, " ")?;
                }
                write!(f, "}}")
            },
            TupleStruct(ref type_, ref fields) => {
                write!(f, "data {} ", type_.name)?;

                for ref param in type_.parameters.iter() {
                    write!(f, "{} ", param.name)?;
                }

                write!(f, "= {}", type_.name)?;

                for ref field in fields.iter() {
                    if field.parameters.is_empty() {
                        write!(f, " {}", field)?;
                    } else {
                        write!(f, " ({})", field)?;
                    }
                }
                Ok(())
            },
            Enum(ref type_, ref constructors) => {
                write!(f, "data {} ", type_.name)?;

                for ref param in type_.parameters.iter() {
                    write!(f, "{} ", param.name)?;
                }

                write!(f, "= ")?;

                for (idx, ref constructor) in constructors.iter().enumerate() {
                    write!(f, "{}", constructor)?;
                    if idx < constructors.len() - 1 {
                        write!(f, " | ")?;
                    }
                }
                Ok(())
            },
        }
    }
}

/// Struct and enums that implement that trait can be part of generated modules.  ToPursType is
/// required to produce a data type *definition*, whereas ToPursConstructor and its corresponding
/// struct PursConstructor are necessary to *use* a type in definitions.
pub trait ToPursType : ToPursConstructor {
    fn to_purs_type() -> PursType;
}

