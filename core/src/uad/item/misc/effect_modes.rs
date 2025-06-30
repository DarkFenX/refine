use crate::{ad, misc::EffectMode, util::RMap};

const DEFAULT_EFFECT_MODE: EffectMode = EffectMode::FullCompliance;

#[derive(Clone)]
pub(crate) struct EffectModes {
    data: RMap<ad::AEffectId, EffectMode>,
}
impl EffectModes {
    pub(in crate::uad::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    // Query methods
    pub(crate) fn get(&self, a_effect_id: &ad::AEffectId) -> &EffectMode {
        self.data.get(a_effect_id).unwrap_or(&DEFAULT_EFFECT_MODE)
    }
    // Modification methods
    pub(crate) fn set(&mut self, a_effect_id: ad::AEffectId, mode: EffectMode) {
        match mode {
            DEFAULT_EFFECT_MODE => self.data.remove(&a_effect_id),
            _ => self.data.insert(a_effect_id, mode),
        };
    }
}
