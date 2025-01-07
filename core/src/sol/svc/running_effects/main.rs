use crate::{
    defs::{EEffectId, SolItemId},
    util::StMapSetL1,
};

#[derive(Clone)]
pub(in crate::sol::svc) struct SolRunningEffects {
    pub(super) data: StMapSetL1<SolItemId, EEffectId>,
}
impl SolRunningEffects {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            data: StMapSetL1::new(),
        }
    }
    // Query methods
    pub(in crate::sol::svc) fn is_running(&self, item_id: &SolItemId, effect_id: &EEffectId) -> bool {
        self.data.get(item_id).any(|v| v == effect_id)
    }
    pub(in crate::sol::svc) fn iter_running(&self, item_id: &SolItemId) -> impl ExactSizeIterator<Item = &EEffectId> {
        self.data.get(item_id)
    }
    // Modification methods
    pub(in crate::sol::svc) fn effects_started(
        &mut self,
        item_id: SolItemId,
        effects: impl ExactSizeIterator<Item = EEffectId>,
    ) {
        self.data.extend_entries(item_id, effects);
    }
    pub(in crate::sol::svc) fn effects_stopped<'a>(
        &mut self,
        item_id: &SolItemId,
        effects: impl Iterator<Item = &'a EEffectId>,
    ) {
        self.data.drain_entries(item_id, effects);
    }
}
