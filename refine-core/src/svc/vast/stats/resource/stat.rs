use crate::Value;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatResource {
    pub used: Value,
    pub output: Option<Value>,
}
