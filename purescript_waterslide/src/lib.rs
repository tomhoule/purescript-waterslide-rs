use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub mod purs_constructor;

pub use purs_constructor::*;

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

pub trait ToPursType : ToPursConstructor {
    fn to_purs_type() -> PursType;
}

impl<T: ToPursConstructor> ToPursConstructor for Vec<T> {
    fn to_purs_constructor() -> PursConstructor {
        PursConstructor {
            name: "Array".to_string(),
            module: None,
            parameters: vec![<T as ToPursConstructor>::to_purs_constructor()],
        }
    }
}

impl<'a, T: ToPursConstructor> ToPursConstructor for &'a [T]
{
    fn to_purs_constructor() -> PursConstructor {
        PursConstructor {
            name: "Array".to_string(),
            module: None,
            parameters: vec![<T as ToPursConstructor>::to_purs_constructor()],
        }
    }
}

impl<T: ToPursConstructor> ToPursConstructor for Option<T> {
    fn to_purs_constructor() -> PursConstructor {
        PursConstructor {
            name: "Maybe".to_string(),
            module: Some("Data.Maybe".to_string()),
            parameters: vec![<T as ToPursConstructor>::to_purs_constructor()],
        }
    }
}

impl<'a> ToPursConstructor for &'a str {
    fn to_purs_constructor() -> PursConstructor {
        PursConstructor {
            name: "String".to_string(),
            module: None,
            parameters: vec![],
        }
    }
}

impl<T, U> ToPursConstructor for (T, U)
    where
    T: ToPursConstructor,
    U: ToPursConstructor,
{
    fn to_purs_constructor() -> PursConstructor {
        PursConstructor {
            name: "Tuple".to_string(),
            module: Some("Data.Tuple".to_string()),
            parameters: vec![
                <T as ToPursConstructor>::to_purs_constructor(),
                <U as ToPursConstructor>::to_purs_constructor(),
            ],
        }
    }
}

impl ToPursConstructor for () {
    fn to_purs_constructor() -> PursConstructor {
        PursConstructor {
            module: Some("Prelude".to_string()),
            name: "Tuple".to_string(),
            parameters: vec![],
        }
    }
}

impl<T: ToPursConstructor> ToPursConstructor for Box<T> {
    fn to_purs_constructor() -> PursConstructor {
        T::to_purs_constructor()
    }
}

impl<'a, T: ToPursConstructor> ToPursConstructor for &'a T {
    fn to_purs_constructor() -> PursConstructor {
        T::to_purs_constructor()
    }
}


// Make that a feature so people can decide on their impls
// enabled by default
macro_rules! purs_primitive_impl {
    ($rust_type:ty, $purs_type:expr, $import:expr) => {
        impl ToPursConstructor for $rust_type {
            fn to_purs_constructor() -> PursConstructor {
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

#[derive(Debug)]
pub struct PursModule {
    name: String,
    imports: BTreeMap<String, Vec<String>>,
    types: Vec<PursType>,
}

#[macro_export]
macro_rules! purs_module {
    ( $name:expr ; $( $p:path ),* ) => {
        {
            let purs_types = vec![
                $( <$p>::to_purs_type() ),*
            ];
            PursModule::new($name, purs_types)
        }
    }
}

impl PursModule {
    /// The `purs_module!` macro is slightly more convenient because it calls `to_purs_type` for
    /// you.
    pub fn new(name: String, types: Vec<PursType>) -> Self {
        let mut imports = BTreeMap::new();
        imports.insert(
            "Data.Generic".to_string(),
            vec!["class Generic".to_string()],
        );

        for type_ in types.iter() {
            match *type_ {
                PursType::Struct(ref name, ref fields) => {
                    Self::accumulate_imports(&mut imports, name);

                    for &(ref _name, ref type_) in fields.iter() {
                        Self::accumulate_imports(&mut imports, type_)
                    }
                },
                PursType::TupleStruct(ref name, ref fields) => {
                    Self::accumulate_imports(&mut imports, name);

                    for field in fields.iter() {
                        Self::accumulate_imports(&mut imports, &field)
                    }
                },
                PursType::Enum(ref name, ref c) => {
                    Self::accumulate_imports(&mut imports, name);

                    for item in c.iter() {
                        Self::accumulate_imports(&mut imports, &item)
                    }
                },
            }
        }
        PursModule {
            name,
            imports,
            types,
        }
    }

    pub fn accumulate_imports(imports: &mut BTreeMap<String, Vec<String>>, type_: &PursConstructor) {
        if let Some(ref import) = type_.module {
            let mut value = imports.entry(import.clone())
                .or_insert_with(|| Vec::new());
            if let None = value.iter().find(|i| **i == type_.name) {
                value.push(type_.name.clone())
            }
        }

        for ref param in type_.parameters.iter() {
            Self::accumulate_imports(imports, &param)
        }
    }
}

impl Display for PursModule {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "module {} where\n\n", self.name)?;

        for (key, value) in self.imports.iter() {
            if key == "PRIM" {
                continue;
            }
            write!(f, "import {} (", key)?;
            for v in value.iter() {
                write!(f, "\n{}", v)?;
            }
            write!(f, "\n)\n")?;
        }
        write!(f, "\n")?;

        for ref type_ in self.types.iter() {
            match *type_ {
                &PursType::TupleStruct(ref constructor_, ref _fields) => {
                    write!(f, "{}\n\n", type_)?;
                    write!(f, "derive instance generic{} :: Generic {}\n\n", constructor_.name, constructor_.name)?;
                },
                &PursType::Struct(ref constructor, ref _fields) => {
                    write!(f, "{}\n\n", type_)?;
                    write!(f, "derive instance generic{} :: Generic {}\n\n", constructor.name, constructor.name)?;
                }
                &PursType::Enum(ref constructor, ref _constructors) => {
                    write!(f, "{}\n\n", type_)?;
                    write!(f, "derive instance generic{} :: Generic {}\n\n", constructor.name, constructor.name)?;
                }
            }
        }
        Ok(())
    }
}
