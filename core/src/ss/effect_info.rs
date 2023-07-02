use super::EffectMode;

pub struct EffectInfo {
    pub running: bool,
    pub mode: EffectMode,
}
impl EffectInfo {
    pub(in crate::ss) fn new(running: bool, mode: EffectMode) -> Self {
        Self { running, mode }
    }
}
