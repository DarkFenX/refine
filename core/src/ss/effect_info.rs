use crate::{consts::EffectMode, defs::EffectId};

pub struct EffectInfo {
    pub effect_id: EffectId,
    pub running: bool,
    pub mode: EffectMode,
}
impl EffectInfo {
    pub(in crate::ss) fn new(effect_id: EffectId, running: bool, mode: EffectMode) -> Self {
        Self {
            effect_id,
            running,
            mode,
        }
    }
}
