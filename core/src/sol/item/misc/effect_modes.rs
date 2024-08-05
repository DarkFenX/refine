use crate::{config::DEFAULT_EFFECT_MODE, defs::EEffectId, sol::SolEffectMode, util::StMap};

#[derive(Clone)]
pub(in crate::sol) struct SolEffectModes {
    data: StMap<EEffectId, SolEffectMode>,
}
impl SolEffectModes {
    pub(in crate::sol::item) fn new() -> Self {
        Self { data: StMap::new() }
    }
    // Query methods
    pub(in crate::sol) fn get(&self, effect_id: &EEffectId) -> &SolEffectMode {
        self.data.get(effect_id).unwrap_or(&DEFAULT_EFFECT_MODE)
    }
    // Modification methods
    pub(in crate::sol) fn set(&mut self, effect_id: EEffectId, mode: SolEffectMode) {
        if mode == DEFAULT_EFFECT_MODE {
            self.data.remove(&effect_id);
        } else {
            self.data.insert(effect_id, mode);
        }
    }
}
