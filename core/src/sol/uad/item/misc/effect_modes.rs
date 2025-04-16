use crate::{ad, config::DEFAULT_EFFECT_MODE, sol::EffectMode, util::RMap};

#[derive(Clone)]
pub(in crate::sol) struct EffectModes {
    data: RMap<ad::AEffectId, EffectMode>,
}
impl EffectModes {
    pub(in crate::sol::uad::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    // Query methods
    pub(in crate::sol) fn get(&self, a_effect_id: &ad::AEffectId) -> &EffectMode {
        self.data.get(a_effect_id).unwrap_or(&DEFAULT_EFFECT_MODE)
    }
    // Modification methods
    pub(in crate::sol) fn set(&mut self, a_effect_id: ad::AEffectId, mode: EffectMode) {
        if mode == DEFAULT_EFFECT_MODE {
            self.data.remove(&a_effect_id);
        } else {
            self.data.insert(a_effect_id, mode);
        }
    }
}
