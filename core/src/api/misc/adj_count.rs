use crate::def::DefCount;

pub struct AdjustableCount {
    pub current: DefCount,
    pub max: DefCount,
    pub overridden: bool,
}
