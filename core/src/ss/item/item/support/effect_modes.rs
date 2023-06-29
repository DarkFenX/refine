use crate::{config::DEFAULT_EFFECT_MODE, consts::EffectMode, defs::EEffectId, util::OptMap};

pub(in crate::ss) struct EffectModes {
    data: OptMap<EEffectId, EffectMode>,
}
impl EffectModes {
    pub(in crate::ss::item::item) fn new() -> Self {
        Self { data: OptMap::new() }
    }
    // Query methods
    pub(in crate::ss) fn get(&self, effect_id: &EEffectId) -> &EffectMode {
        self.data.get(effect_id).unwrap_or(&DEFAULT_EFFECT_MODE)
    }
    // Modification methods
    pub(in crate::ss) fn set(&mut self, effect_id: EEffectId, mode: EffectMode) {
        self.data.insert(effect_id, mode)
    }
    pub(in crate::ss) fn unset(&mut self, effect_id: &EEffectId) {
        self.data.remove(effect_id)
    }
}
