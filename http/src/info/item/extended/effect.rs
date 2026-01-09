use serde_tuple::Serialize_tuple;

use crate::shared::HEffectMode;

#[derive(Serialize_tuple)]
pub(in crate::info::item::extended) struct HEffect {
    running: bool,
    mode: HEffectMode,
}
impl HEffect {
    pub(in crate::info::item::extended) fn from_core(core_effect_info: rc::EffectInfo) -> Self {
        Self {
            running: core_effect_info.running,
            mode: HEffectMode::from_core(core_effect_info.mode),
        }
    }
}
