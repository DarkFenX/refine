use crate::num::CountNz;

#[derive(Copy, Clone)]
pub struct FighterCountInfo {
    pub current: CountNz,
    pub max: CountNz,
    pub overridden: bool,
}
