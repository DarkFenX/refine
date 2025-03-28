use crate::shared::HEffectMode;

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HEffect {
    pub(crate) running: bool,
    pub(crate) mode: HEffectMode,
}
impl From<&rc::EffectInfo> for HEffect {
    fn from(core_effect_info: &rc::EffectInfo) -> Self {
        Self {
            running: core_effect_info.running,
            mode: (&core_effect_info.mode).into(),
        }
    }
}
