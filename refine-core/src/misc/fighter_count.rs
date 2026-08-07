use crate::num::CountNz;

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct FighterCountInfo {
    /// Current count of fighters in this squad; different from max only if overridden.
    pub current: CountNz,
    /// Max count of fighters this squad supports.
    pub max: CountNz,
    /// True if current count of fighters in a squad is set via override.
    pub overridden: bool,
}
