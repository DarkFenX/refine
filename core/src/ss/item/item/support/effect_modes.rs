use std::collections::HashMap;

use crate::{config::DEFAULT_EFFECT_MODE, defs::EEffectId, ss::EffectMode};

pub(in crate::ss) struct EffectModes {
    data: HashMap<EEffectId, EffectMode>,
}
impl EffectModes {
    pub(in crate::ss::item::item) fn new() -> Self {
        Self { data: HashMap::new() }
    }
    // Query methods
    pub(in crate::ss) fn get(&self, effect_id: &EEffectId) -> &EffectMode {
        self.data.get(effect_id).unwrap_or(&DEFAULT_EFFECT_MODE)
    }
    // Modification methods
    pub(in crate::ss) fn set(&mut self, effect_id: EEffectId, mode: EffectMode) {
        if mode == DEFAULT_EFFECT_MODE {
            self.data.remove(&effect_id);
            // This struct is not intended to be used and re-used often,
            // free up memory when we don't have elements
            if self.data.is_empty() {
                self.data = HashMap::new();
            }
        } else {
            self.data.insert(effect_id, mode);
        }
    }
}
