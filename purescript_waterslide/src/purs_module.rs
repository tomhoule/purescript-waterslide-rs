use std::collections::BTreeMap;
use purs_constructor::*;
use purs_type::*;
use std::fmt::{Display, Formatter};

/// Represents a Purescript module with a name, imports and multiple data types declarations. It is
/// most easily generated with the `purs_module!` macro. You can then use the `Display`
/// implementations.
#[derive(Debug)]
pub struct PursModule {
    name: String,
    imports: BTreeMap<String, Vec<String>>,
    types: Vec<PursType>,
}

impl PursModule {
    /// The `purs_module!` macro is slightly more convenient because it calls `as_purs_type` for
    /// you.
    pub fn new(name: String, types: Vec<PursType>) -> Self {
        let mut imports = BTreeMap::new();
        imports.insert(
            "Data.Generic".to_string(),
            vec!["class Generic".to_string()],
        );

        for type_ in &types {
            match *type_ {
                PursType::Struct(ref name, ref fields) => {
                    Self::accumulate_imports(&mut imports, name);

                    for &(ref _name, ref type_) in fields.iter() {
                        Self::accumulate_imports(&mut imports, type_)
                    }
                }
                PursType::TupleStruct(ref name, ref fields) => {
                    Self::accumulate_imports(&mut imports, name);

                    for field in fields.iter() {
                        Self::accumulate_imports(&mut imports, field)
                    }
                }
                PursType::Enum(ref name, ref c) => {
                    Self::accumulate_imports(&mut imports, name);

                    for item in c.iter() {
                        Self::accumulate_imports(&mut imports, item)
                    }
                }
            }
        }
        PursModule {
            name,
            imports,
            types,
        }
    }

    fn accumulate_imports(imports: &mut BTreeMap<String, Vec<String>>, type_: &PursConstructor) {
        if let Some(ref import) = type_.module {
            let value = imports.entry(import.clone()).or_insert_with(Vec::new);
            if value.iter().find(|i| **i == type_.name).is_none() {
                value.push(type_.name.clone())
            }
        }

        for param in &type_.parameters {
            Self::accumulate_imports(imports, param)
        }
    }
}

impl Display for PursModule {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "module {} where\n\n", self.name)?;

        for (key, value) in &self.imports {
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

        let types = &self.types;
        let output: Vec<String> = types
            .into_iter()
            .map(|type_| match *type_ {
                PursType::TupleStruct(ref constructor, ref _fields) => format!(
                    "{}\n\nderive instance generic{} :: Generic {}\n",
                    type_,
                    constructor.name,
                    constructor.name
                ),
                PursType::Struct(ref constructor, ref _fields) => format!(
                    "{}\n\nderive instance generic{} :: Generic {}\n",
                    type_,
                    constructor.name,
                    constructor.name
                ),
                PursType::Enum(ref constructor, ref _constructors) => format!(
                    "{}\n\nderive instance generic{} :: Generic {}\n",
                    type_,
                    constructor.name,
                    constructor.name
                ),
            })
            .collect();
        write!(f, "{}", output.join("\n"))?;
        Ok(())
    }
}

/// Use this macro to generate purescript modules. It takes a module name (a String) and a
/// comma-separated list of types you want to include in the module.
///
/// `purs_module!("Data.Pasta.Ingredients".to_string() ; TomatoSauce, OliveOil, Spinach,
/// Sauce<void::Void>, Butter);`
///
/// Note the usage of the `Void` type from the `void` crate as a type argument. Since the type
/// arguments are not used when deriving `AsPursType`, any other type should work here.
#[macro_export]
macro_rules! purs_module {
    ( $name:expr ; $( $p:path ),* ) => {
        {
            let purs_types = vec![
                $( <$p>::as_purs_type() ),*
            ];
            PursModule::new($name, purs_types)
        }
    }
}
