use super::SsEffectMode;

pub struct SsEffectInfo {
    pub running: bool,
    pub mode: SsEffectMode,
}
impl SsEffectInfo {
    pub(in crate::ss) fn new(running: bool, mode: SsEffectMode) -> Self {
        Self { running, mode }
    }
}
