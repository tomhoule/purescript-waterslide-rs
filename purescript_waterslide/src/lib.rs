use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import {
    pub type_module: &'static str,
}

impl ::std::cmp::PartialOrd for Import {
    fn partial_cmp(&self, other: &Import) -> Option<::std::cmp::Ordering> {
        self.type_module.partial_cmp(other.type_module)
    }
}

impl ::std::cmp::Ord for Import {
    fn cmp(&self, other: &Import) -> ::std::cmp::Ordering {
        self.type_module.cmp(other.type_module)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordConstructor {
    pub import: Option<Import>,
    pub name: String,
    pub arguments: Vec<(String, PursType)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SeqConstructor {
    pub import: Option<Import>,
    pub name: String,
    pub arguments: Vec<PursType>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Constructor {
    Seq(SeqConstructor),
    Record(RecordConstructor),
}

impl Constructor {
    fn get_constructor_name(&self) -> String {
        match *self {
            Constructor::Seq(ref c) => c.name.clone(),
            Constructor::Record(ref c) => c.name.clone(),
        }
    }

    fn get_import(&self) -> &Option<Import> {
        match *self {
            Constructor::Seq(ref c) => &c.import,
            Constructor::Record(ref c) => &c.import,
        }
    }

    fn get_name(&self) -> String {
        match *self {
            Constructor::Seq(ref c) => {
                let mut s = String::new();

                if !c.arguments.is_empty() {
                    s.push_str("(");
                }

                s.push_str(&c.name);

                for ref arg in c.arguments.iter() {

                    s.push(' ');
                    s.push_str(&arg.get_name());
                }

                if !c.arguments.is_empty() {
                    s.push_str(")");
                }

                s
            }
            Constructor::Record(ref c) => format!("{}", c.name),
        }
    }

    fn get_children(&self) -> Vec<PursType> {
        match *self {
            Constructor::Seq(ref c) => c.arguments.clone(),
            Constructor::Record(ref c) => c.arguments
                .clone()
                .into_iter()
                .map(|(_, arg)| arg)
                .collect(),
        }
    }
}

impl Display for Constructor {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match *self {
            Constructor::Seq(ref c) => write!(f, "{}", c),
            Constructor::Record(ref c) => write!(f, "{}", c),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PursType {
    Struct(Constructor),
    Enum(String, Vec<Constructor>),
    Leaf(Import, String),
}

impl PursType {
    fn get_name(&self) -> String {
        use PursType::*;
        match *self {
            Struct(ref constructor) => constructor.get_name(),
            Enum(ref name, _) => format!("{}", name),
            Leaf(ref _import, ref name) => format!("{}", name),
        }
    }
}

impl Display for SeqConstructor {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.name)?;
        for ref arg in self.arguments.iter() {
            write!(f, " {}", arg.get_name())?;
        }
        Ok(())
    }
}

impl Display for RecordConstructor {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "{} {{ ", self.name)?;
        for (idx, &(ref name, ref type_)) in self.arguments.iter().enumerate() {
            write!(f, "{} :: {}", name, type_.get_name())?;
            if idx < (self.arguments.len() - 1) {
                write!(f, ",")?;
            }
            write!(f, " ")?;
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
            Struct(Seq(ref constructor)) => write!(f, "{}", constructor),
            Enum(ref _name, ref constructors) => {
                for (idx, ref constructor) in constructors.iter().enumerate() {
                    write!(f, "{}", constructor)?;
                    if idx < constructors.len() - 1 {
                        write!(f, " | ")?;
                    }
                }
                Ok(())
            },
            Leaf(_, ref ty) => {
                write!(f, "{}", ty)?;
                Ok(())
            },
        }
    }
}

pub trait ToPursType {
    fn to_purs_type() -> PursType;
}


impl<T> ToPursType for Vec<T>
where
    T: ToPursType,
{
    fn to_purs_type() -> PursType {
        PursType::Struct(Constructor::Seq(SeqConstructor {
            import: None,
            name: "Array".to_string(),
            arguments: vec![<T as ToPursType>::to_purs_type()],
        }))
    }
}

impl<'a, T> ToPursType for &'a [T]
where T: ToPursType
{
    fn to_purs_type() -> PursType {
        PursType::Struct(Constructor::Seq(SeqConstructor {
            import: None,
            name: "Array".to_string(),
            arguments: vec![<T as ToPursType>::to_purs_type()]
        }))
    }
}

impl<T> ToPursType for Option<T>
where
    T: ToPursType,
{
    fn to_purs_type() -> PursType {
        PursType::Struct(Constructor::Seq(SeqConstructor {
            import: Some(Import {
                type_module: "Data.Maybe",
            }),
            name: "Maybe".to_string(),
            arguments: vec![<T as ToPursType>::to_purs_type()],
        }))
    }
}

impl<'a> ToPursType for &'a str
{
    fn to_purs_type() -> PursType {
        PursType::Leaf(Import { type_module: "PRIM" }, "String".to_string())
    }
}

impl<T, U> ToPursType for (T, U)
where
    T: ToPursType,
    U: ToPursType,
{
    fn to_purs_type() -> PursType {
        PursType::Struct(Constructor::Seq(SeqConstructor {
            import: Some(Import {
                type_module: "Data.Tuple",
            }),
            name: "Tuple".to_string(),
            arguments: vec![
                <T as ToPursType>::to_purs_type(),
                <U as ToPursType>::to_purs_type(),
            ],
        }))
    }
}

impl ToPursType for () {
    fn to_purs_type() -> PursType {
        PursType::Struct(Constructor::Seq(SeqConstructor {
            import: Some(Import {
                type_module: "Prelude",
            }),
            name: "Tuple".to_string(),
            arguments: vec![],
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

const PRIM: Import = Import {
    type_module: "PRIM",
};

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
    imports: BTreeMap<Import, Vec<String>>,
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
            Import {
                type_module: "Data.Generic",
            },
            vec!["class Generic".to_string()],
        );

        for type_ in types.iter() {
            Self::accumulate_imports(&mut imports, type_)
        }
        PursModule {
            name,
            imports,
            types,
        }
    }

    pub fn accumulate_imports(imports: &mut BTreeMap<Import, Vec<String>>, type_: &PursType) {
        use PursType::*;

        match *type_ {
            Struct(ref constructor) => {
                if let Some(ref import) = *constructor.get_import() {
                    {
                        let mut value = imports.entry(import.clone()).or_insert_with(|| Vec::new());
                        let name = constructor.get_constructor_name();
                        if let None = value.iter().find(|i| **i == name) {
                            value.push(name.clone());
                        }
                    }
                }
                for ref inner in constructor.get_children().iter() {
                    Self::accumulate_imports(imports, inner);
                }
            }
            Enum(_, ref constructors) => for ref constructor in constructors.iter() {
                if let &Some(ref import) = constructor.get_import() {
                    {
                        let mut value = imports.entry(import.clone()).or_insert_with(|| Vec::new());
                        value.push(constructor.get_constructor_name().clone());
                        let name = constructor.get_constructor_name();
                        if let None = value.iter().find(|i| **i == name) {
                            value.push(name.clone());
                        }
                    }
                }
                for ref inner in constructor.get_children().iter() {
                    Self::accumulate_imports(imports, inner);
                }
            },
            Leaf(ref import, ref name) => {
                let mut value = imports.entry(import.clone()).or_insert_with(|| Vec::new());
                if let None = value.iter().find(|i| *i == name) {
                    value.push(name.clone());
                }
            }
        }
    }
}

impl Display for PursModule {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "module {} where\n\n", self.name)?;

        for (key, value) in self.imports.iter() {
            if key.type_module == "PRIM" {
                continue;
            }
            write!(f, "import {} (", key.type_module)?;
            for v in value.iter() {
                write!(f, "\n{}", v)?;
            }
            write!(f, "\n)\n")?;
        }
        write!(f, "\n")?;

        for ref type_ in self.types.iter() {
            match *type_ {
                &PursType::Leaf(_, _) => panic!("Leaf types cannot be at the module top-level"),
                &PursType::Struct(ref constructor) => {
                    let name = constructor.get_name();
                    write!(f, "data {} = {}\n\n", name, constructor)?;
                    write!(f, "derive instance generic{} :: Generic {}\n\n", name, name)?;
                }
                &PursType::Enum(ref name, ref _constructors) => {
                    write!(f, "data {} = {}\n\n", name, type_)?;
                    write!(f, "derive instance generic{} :: Generic {}\n\n", name, name)?;
                }
            }
        }
        Ok(())
    }
}
