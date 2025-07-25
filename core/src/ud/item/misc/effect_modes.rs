use crate::{ad, misc::EffectMode, util::RMap};

const DEFAULT_EFFECT_MODE: EffectMode = EffectMode::FullCompliance;

#[derive(Clone)]
pub(in crate::ud::item) struct EffectModes {
    data: RMap<ad::AEffectId, EffectMode>,
}
impl EffectModes {
    pub(in crate::ud::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    // Query methods
    pub(in crate::ud::item) fn get(&self, a_effect_id: &ad::AEffectId) -> EffectMode {
        match self.data.get(a_effect_id) {
            Some(effect_mode) => *effect_mode,
            None => DEFAULT_EFFECT_MODE,
        }
    }
    // Modification methods
    pub(in crate::ud::item) fn set(&mut self, a_effect_id: ad::AEffectId, mode: EffectMode) {
        match mode {
            DEFAULT_EFFECT_MODE => self.data.remove(&a_effect_id),
            _ => self.data.insert(a_effect_id, mode),
        };
    }
}
