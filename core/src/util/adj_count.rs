use crate::defs::Count;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdjustableCount {
    pub current: Count,
    pub max: Count,
}
