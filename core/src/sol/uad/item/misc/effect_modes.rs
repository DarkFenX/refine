use crate::{ad, config::DEFAULT_EFFECT_MODE, sol::EffectMode, util::HMap};

#[derive(Clone)]
pub(in crate::sol) struct EffectModes {
    data: HMap<ad::AEffectId, EffectMode>,
}
impl EffectModes {
    pub(in crate::sol::uad::item) fn new() -> Self {
        Self { data: HMap::new() }
    }
    // Query methods
    pub(in crate::sol) fn get(&self, effect_id: &ad::AEffectId) -> &EffectMode {
        self.data.get(effect_id).unwrap_or(&DEFAULT_EFFECT_MODE)
    }
    // Modification methods
    pub(in crate::sol) fn set(&mut self, effect_id: ad::AEffectId, mode: EffectMode) {
        if mode == DEFAULT_EFFECT_MODE {
            self.data.remove(&effect_id);
        } else {
            self.data.insert(effect_id, mode);
        }
    }
}
