/// Auxiliary entity for "primitive" data.
#[derive(Clone)]
pub enum EPrimitive {
    /// Represents absence of a value.
    Null,
    /// Represents a boolean value.
    Bool(bool),
    /// Represents an integer number value.
    Int(i32),
    /// Represents a float number value.
    Float(f64),
    /// Represents a string value.
    String(String),
}
