use std::collections::BTreeMap;
use std::fmt::{Formatter, Display};

#[derive(Debug, PartialEq)]
pub struct Import {
    pub type_module: &'static str,
}

#[derive(Debug, PartialEq)]
pub struct RecordConstructor {
    pub import: Option<Import>,
    pub name: String,
    pub arguments: Vec<(String, PursType)>,
}

#[derive(Debug, PartialEq)]
pub struct SeqConstructor {
    pub import: Option<Import>,
    pub name: String,
    pub arguments: Vec<PursType>,
}

#[derive(Debug, PartialEq)]
pub enum Constructor {
    Seq(SeqConstructor),
    Record(RecordConstructor),
}

impl Display for Constructor {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match *self {
            Constructor::Seq(ref c) => {
                write!(f, "{}", c)
            },
            Constructor::Record(ref c) => {
                write!(f, "{}", c)
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PursType {
    Struct(Constructor),
    Enum(String, Vec<Constructor>),
    Leaf(Import, String),
}

impl Display for SeqConstructor {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "{} ", self.name)?;
        for ref arg in self.arguments.iter() {
            write!(f, "{}", arg)?;
        }
        Ok(())
    }
}

impl Display for RecordConstructor {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "{} {{ ", self.name)?;
        for &(ref name, ref type_) in self.arguments.iter() {
            write!(f, "{} :: {}, ", name, type_)?;
        }
        write!(f, "}}")
    }
}

impl Display for PursType {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        use PursType::*;
        use Constructor::*;

        match *self {
            Struct(Record(ref constructor)) => write!(f, "{}", constructor),
            Enum(ref name, ref constructors) => {
                for ref constructor in constructors.iter() {
                    write!(f, "{}", constructor)?;
                    write!(f, "| ")?;
                }
                Ok(())
            }
            Leaf(_, ref ty) => {
                write!(f, "{}", ty)?;
                Ok(())
            },
            _ => unimplemented!(),
        }
    }
}

pub trait ToPursType {
    fn to_purs_type() -> PursType;
}


impl<T> ToPursType for Vec<T>
where T: ToPursType
{
    fn to_purs_type() -> PursType {
        PursType::Struct(Constructor::Seq(SeqConstructor {
            import: Some(Import { type_module: "Data.Array" }),
            name: "Array".to_string(),
            arguments: vec![<T as ToPursType>::to_purs_type()]
        }))
    }
}

impl<T, U> ToPursType for (T, U)
where T: ToPursType,
      U: ToPursType
{
    fn to_purs_type() -> PursType {
        PursType::Struct(Constructor::Seq(SeqConstructor {
            import: Some(Import { type_module: "Data.Tuple" }),
            name: "Tuple".to_string(),
            arguments: vec![
                <T as ToPursType>::to_purs_type(),
                <U as ToPursType>::to_purs_type()
            ]
        }))
    }
}

impl ToPursType for ()
{
    fn to_purs_type() -> PursType {
        PursType::Struct(Constructor::Seq(SeqConstructor {
            import: Some(Import { type_module: "Prelude" }),
            name: "Tuple".to_string(),
            arguments: vec![]
        }))
    }
}

// Make that a feature so people can decide on their impls
// enabled by default
macro_rules! purs_primitive_impl {
    ($rust_type:ty, $purs_type:expr, $import:expr) => {
        impl ToPursType for $rust_type {
            fn to_purs_type() -> PursType {
                PursType::Leaf($import, $purs_type.to_string())
            }
        }
    }
}

const PRIM: Import = Import { type_module: "PRIM" };

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

struct Module {
    name: String,
    imports: BTreeMap<Import, Vec<String>>,
    types: Vec<PursType>,
}
