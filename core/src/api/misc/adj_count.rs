use crate::def::Count;

pub struct AdjustableCount {
    pub current: Count,
    pub max: Count,
    pub overridden: bool,
}
