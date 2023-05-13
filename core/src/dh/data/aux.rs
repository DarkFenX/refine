use crate::defs::{ReeFloat, ReeInt};

/// Auxiliary entity for "primitive" data.
#[derive(Debug)]
pub enum Primitive {
    /// Represents absence of a value.
    Null,
    /// Represents a boolean value.
    Bool(bool),
    /// Represents an integer number value.
    Int(ReeInt),
    /// Represents a float number value.
    Float(ReeFloat),
    /// Represents a string value.
    String(String),
}
