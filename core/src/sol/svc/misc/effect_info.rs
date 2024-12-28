use crate::sol::SolEffectMode;

pub struct SolEffectInfo {
    pub running: bool,
    pub mode: SolEffectMode,
}
impl SolEffectInfo {
    pub(in crate::sol) fn new(running: bool, mode: SolEffectMode) -> Self {
        Self { running, mode }
    }
}
