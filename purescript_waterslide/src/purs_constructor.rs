
pub trait ToPursConstructor {
    fn to_purs_constructor() -> PursConstructor;
}

pub struct PursConstructor {
    pub module: Option<String>,
    pub name: String,
    pub parameters: Vec<PursConstructor>,
}
