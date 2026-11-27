use crate::Value;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatResource {
    pub used: Value,
    pub output: Option<Value>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct StatFighterBay {
    /// Volume taken by fighters which are strictly in "in bay" state (not loaded into tubes).
    pub used_inbay: Value,
    /// Volume taken by all fighters, regardless of their state.
    pub used_all: Value,
    pub output: Option<Value>,
}
