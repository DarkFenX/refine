use crate::{config::DEFAULT_EFFECT_MODE, defs::EEffectId, ss::SsEffectMode, util::StMap};

pub(in crate::ss) struct SsEffectModes {
    data: StMap<EEffectId, SsEffectMode>,
}
impl SsEffectModes {
    pub(in crate::ss::item) fn new() -> Self {
        Self { data: StMap::new() }
    }
    // Query methods
    pub(in crate::ss) fn get(&self, effect_id: &EEffectId) -> &SsEffectMode {
        self.data.get(effect_id).unwrap_or(&DEFAULT_EFFECT_MODE)
    }
    // Modification methods
    pub(in crate::ss) fn set(&mut self, effect_id: EEffectId, mode: SsEffectMode) {
        if mode == DEFAULT_EFFECT_MODE {
            self.data.remove(&effect_id);
        } else {
            self.data.insert(effect_id, mode);
        }
    }
}
