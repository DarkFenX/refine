#[derive(Clone)]
pub enum EPrimitive {
    Null,
    Bool(bool),
    Int(i32),
    Float(f64),
    String(String),
}
