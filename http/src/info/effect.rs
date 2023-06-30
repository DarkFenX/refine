use crate::shared::HEffectMode;

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HEffect {
    pub(crate) running: bool,
    pub(crate) mode: HEffectMode,
}
impl HEffect {
    fn new(running: bool, mode: HEffectMode) -> Self {
        Self { running, mode }
    }
}
impl From<&rc::EffectInfo> for HEffect {
    fn from(core_effect_info: &rc::EffectInfo) -> Self {
        Self::new(core_effect_info.running, (&core_effect_info.mode).into())
    }
}
