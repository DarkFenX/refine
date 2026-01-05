use crate::{
    ad::AEffectId,
    misc::EffectMode,
    rd::{REffectId, Src},
    util::RMap,
};

const DEFAULT_EFFECT_MODE: EffectMode = EffectMode::FullCompliance;

#[derive(Clone)]
pub(in crate::ud::item) struct UEffectModes {
    by_aid: RMap<AEffectId, EffectMode>,
    pub(super) by_rid: RMap<REffectId, EffectMode>,
}
impl UEffectModes {
    pub(in crate::ud::item) fn new() -> Self {
        Self {
            by_aid: RMap::new(),
            by_rid: RMap::new(),
        }
    }
    // Query methods
    pub(in crate::ud::item) fn get_by_rid(&self, effect_rid: &REffectId) -> EffectMode {
        match self.by_rid.get(effect_rid) {
            Some(effect_mode) => *effect_mode,
            None => DEFAULT_EFFECT_MODE,
        }
    }
    pub(in crate::ud::item) fn get_by_aid(&self, effect_aid: &AEffectId) -> EffectMode {
        match self.by_aid.get(effect_aid) {
            Some(effect_mode) => *effect_mode,
            None => DEFAULT_EFFECT_MODE,
        }
    }
    // Modification methods
    pub(in crate::ud::item) fn set_by_aid(&mut self, effect_aid: AEffectId, effect_mode: EffectMode, src: &Src) {
        match effect_mode {
            DEFAULT_EFFECT_MODE => {
                self.by_aid.remove(&effect_aid);
                if let Some(effect_rid) = src.get_effect_rid_by_aid(&effect_aid) {
                    self.by_rid.remove(&effect_rid);
                }
            }
            _ => {
                self.by_aid.insert(effect_aid, effect_mode);
                if let Some(effect_rid) = src.get_effect_rid_by_aid(&effect_aid) {
                    self.by_rid.insert(effect_rid, effect_mode);
                }
            }
        };
    }
    pub(in crate::ud::item) fn update_rids(&mut self, src: &Src) {
        self.by_rid.clear();
        for (effect_aid, effect_mode) in self.by_aid.iter() {
            if let Some(effect_rid) = src.get_effect_rid_by_aid(effect_aid) {
                self.by_rid.insert(effect_rid, *effect_mode);
            }
        }
    }
}
