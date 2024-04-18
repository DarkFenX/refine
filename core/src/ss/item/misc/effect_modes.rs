use crate::{config::DEFAULT_EFFECT_MODE, defs::EEffectId, ss::EffectMode, util::StMap};

pub(in crate::ss) struct EffectModes {
    data: StMap<EEffectId, EffectMode>,
}
impl EffectModes {
    pub(in crate::ss::item) fn new() -> Self {
        Self { data: StMap::new() }
    }
    // Query methods
    pub(in crate::ss) fn get(&self, effect_id: &EEffectId) -> &EffectMode {
        self.data.get(effect_id).unwrap_or(&DEFAULT_EFFECT_MODE)
    }
    // Modification methods
    pub(in crate::ss) fn set(&mut self, effect_id: EEffectId, mode: EffectMode) {
        if mode == DEFAULT_EFFECT_MODE {
            self.data.remove(&effect_id);
        } else {
            self.data.insert(effect_id, mode);
        }
    }
}
