use crate::{PValue, rd::REffectId};

pub(super) struct SelfKillerEffectInfo {
    pub(super) effect_rid: REffectId,
    pub(super) active_duration: PValue,
}

pub(super) struct SelfKillerItemInfo {
    effect: Option<SelfKillerEffectInfo>,
}
impl SelfKillerItemInfo {
    pub(super) fn new() -> Self {
        Self { effect: None }
    }
    pub(super) fn push(&mut self, effect: SelfKillerEffectInfo) {
        match &self.effect {
            Some(stored) => {
                if effect.active_duration < stored.active_duration {
                    self.effect = Some(effect);
                }
            }
            None => self.effect = Some(effect),
        }
    }
    pub(super) fn get_effect_rid(&self) -> Option<REffectId> {
        self.effect.as_ref().map(|v| v.effect_rid)
    }
}
