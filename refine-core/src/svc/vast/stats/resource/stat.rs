use crate::num::Value;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
pub struct StatResource {
    pub used: Value,
    pub output: Option<Value>,
}
