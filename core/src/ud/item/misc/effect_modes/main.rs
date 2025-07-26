use crate::{ad::AEffectId, misc::EffectMode, rd::REffectKey, src::Src, util::RMap};

const DEFAULT_EFFECT_MODE: EffectMode = EffectMode::FullCompliance;

#[derive(Clone)]
pub(in crate::ud::item) struct EffectModes {
    by_id: RMap<AEffectId, EffectMode>,
    pub(super) by_key: RMap<REffectKey, EffectMode>,
}
impl EffectModes {
    pub(in crate::ud::item) fn new() -> Self {
        Self {
            by_id: RMap::new(),
            by_key: RMap::new(),
        }
    }
    // Query methods
    pub(in crate::ud::item) fn get_by_key(&self, effect_key: &REffectKey) -> EffectMode {
        match self.by_key.get(effect_key) {
            Some(effect_mode) => *effect_mode,
            None => DEFAULT_EFFECT_MODE,
        }
    }
    pub(in crate::ud::item) fn get_by_id(&self, effect_id: &AEffectId) -> EffectMode {
        match self.by_id.get(effect_id) {
            Some(effect_mode) => *effect_mode,
            None => DEFAULT_EFFECT_MODE,
        }
    }
    // Modification methods
    pub(in crate::ud::item) fn set_by_id(&mut self, effect_id: AEffectId, effect_mode: EffectMode, src: &Src) {
        match effect_mode {
            DEFAULT_EFFECT_MODE => {
                self.by_id.remove(&effect_id);
                if let Some(effect_key) = src.get_effect_key_by_id(&effect_id) {
                    self.by_key.remove(&effect_key);
                }
            }
            _ => {
                self.by_id.insert(effect_id, effect_mode);
                if let Some(effect_key) = src.get_effect_key_by_id(&effect_id) {
                    self.by_key.insert(effect_key, effect_mode);
                }
            }
        };
    }
    pub(in crate::ud::item) fn update_keys(&mut self, src: &Src) {
        self.by_key.clear();
        for (effect_id, effect_mode) in self.by_id.iter() {
            if let Some(effect_key) = src.get_effect_key_by_id(effect_id) {
                self.by_key.insert(effect_key, *effect_mode);
            }
        }
    }
}
