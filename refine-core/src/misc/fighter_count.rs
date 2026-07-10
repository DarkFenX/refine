use crate::num::FighterCount;

#[derive(Copy, Clone)]
pub struct FighterCountInfo {
    pub current: FighterCount,
    pub max: FighterCount,
    pub overridden: bool,
}
