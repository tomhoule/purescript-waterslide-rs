use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub mod purs_constructor;

pub use purs_constructor::*;

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
    pub arguments: Vec<(String, PursConstructor)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SeqConstructor {
    pub import: Option<Import>,
    pub name: String,
    pub arguments: Vec<PursConstructor>,
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
                    s.push_str(&arg.name);
                }

                if !c.arguments.is_empty() {
                    s.push_str(")");
                }

                s
            }
            Constructor::Record(ref c) => format!("{}", c.name),
        }
    }

    fn get_children(&self) -> Vec<PursConstructor> {
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
            if arg.parameters.is_empty() {
                write!(f, " {}", arg)?;
            } else {
                write!(f, " ({})", arg)?;
            }
        }
        Ok(())
    }
}

impl Display for RecordConstructor {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "{} {{ ", self.name)?;
        for (idx, &(ref name, ref type_)) in self.arguments.iter().enumerate() {
            write!(f, "{} :: {}", name, &type_)?;
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

impl<T> ToPursType for Vec<T>
where
    T: ToPursType,
{
    fn to_purs_type() -> PursType {
        PursType::Struct(Constructor::Seq(SeqConstructor {
            import: None,
            name: "Array".to_string(),
            arguments: vec![<T as ToPursConstructor>::to_purs_constructor()],
        }))
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

impl<'a, T> ToPursType for &'a [T]
where T: ToPursType
{
    fn to_purs_type() -> PursType {
        PursType::Struct(Constructor::Seq(SeqConstructor {
            import: None,
            name: "Array".to_string(),
            arguments: vec![<T as ToPursConstructor>::to_purs_constructor()]
        }))
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
            arguments: vec![<T as ToPursConstructor>::to_purs_constructor()],
        }))
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

impl<'a> ToPursType for &'a str
{
    fn to_purs_type() -> PursType {
        PursType::Leaf(Import { type_module: "PRIM" }, "String".to_string())
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
                <T as ToPursConstructor>::to_purs_constructor(),
                <U as ToPursConstructor>::to_purs_constructor(),
            ],
        }))
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

impl ToPursType for () {
    fn to_purs_type() -> PursType {
        PursType::Struct(Constructor::Seq(SeqConstructor {
            import: Some(Import {
                type_module: "Prelude",
            }),
            name: "Unit".to_string(),
            arguments: vec![],
        }))
    }
}

impl<T: ToPursConstructor> ToPursConstructor for Box<T> {
    fn to_purs_constructor() -> PursConstructor {
        T::to_purs_constructor()
    }
}

impl<T: ToPursType> ToPursType for Box<T> {
    fn to_purs_type() -> PursType {
        T::to_purs_type()
    }
}

impl<'a, T: ToPursConstructor> ToPursConstructor for &'a T {
    fn to_purs_constructor() -> PursConstructor {
        T::to_purs_constructor()
    }
}

impl<'a, T: ToPursType> ToPursType for &'a T {
    fn to_purs_type() -> PursType {
        T::to_purs_type()
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

        impl ToPursType for $rust_type {
            fn to_purs_type() -> PursType {
                PursType::Leaf(Import { type_module: $import }, $purs_type.to_string())
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
                PursType::Struct(ref c) => {
                    c.get_children()
                        .iter()
                        .map(|constr| {
                            Self::accumulate_imports(&mut imports, constr)
                        });
                },
                PursType::Enum(_, ref c) => {
                    for item in c.iter() {
                        for constructor in item.get_children().iter() {
                            Self::accumulate_imports(&mut imports, &constructor)
                        }
                    }
                },
                PursType::Leaf(_, _) => ()
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
    }

        // match *type_ {
        //     Struct(ref constructor) => {
        //         if let Some(ref import) = *constructor.get_import() {
        //             {
        //                 let mut value = imports.entry(import.clone()).or_insert_with(|| Vec::new());
        //                 let name = constructor.get_constructor_name();
        //                 if let None = value.iter().find(|i| **i == name) {
        //                     value.push(name.clone());
        //                 }
        //             }
        //         }
        //         for ref inner in constructor.get_children().iter() {
        //             Self::accumulate_imports(imports, inner);
        //         }
        //     }
        //     Enum(_, ref constructors) => for ref constructor in constructors.iter() {
        //         if let &Some(ref import) = constructor.get_import() {
        //             {
        //                 let mut value = imports.entry(import.clone()).or_insert_with(|| Vec::new());
        //                 value.push(constructor.get_constructor_name().clone());
        //                 let name = constructor.get_constructor_name();
        //                 if let None = value.iter().find(|i| **i == name) {
        //                     value.push(name.clone());
        //                 }
        //             }
        //         }
        //         for ref inner in constructor.get_children().iter() {
        //             Self::accumulate_imports(imports, inner);
        //         }
        //     },
        //     Leaf(ref import, ref name) => {
        //         let mut value = imports.entry(import.type_module.clone()).or_insert_with(|| Vec::new());
        //         if let None = value.iter().find(|i| *i == name) {
        //             value.push(name.clone());
        //         }
        //     }
        // }
    // }
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
