use crate::num::CountNz;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct FighterCountInfo {
    pub current: CountNz,
    pub max: CountNz,
    pub overridden: bool,
}
