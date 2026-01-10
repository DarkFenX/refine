pub struct EEffectMod {
    pub func: String,
    pub args: Vec<EEffectModArg>,
}

pub struct EEffectModArg {
    pub name: String,
    pub value: EPrimitive,
}

#[derive(Clone)]
pub enum EPrimitive {
    Null,
    Bool(bool),
    Int(i32),
    Float(f64),
    String(String),
}
