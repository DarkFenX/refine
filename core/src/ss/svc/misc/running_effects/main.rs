use crate::{
    defs::{EEffectId, SsItemId},
    util::StMapSetL1,
};

pub(in crate::ss::svc) struct RunningEffects {
    pub(super) data: StMapSetL1<SsItemId, EEffectId>,
}
impl RunningEffects {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            data: StMapSetL1::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc) fn is_running(&self, item_id: &SsItemId, effect_id: &EEffectId) -> bool {
        self.data.get(item_id).any(|v| v == effect_id)
    }
    // Modification methods
    pub(in crate::ss::svc) fn effects_started<I>(&mut self, item_id: SsItemId, effects: I)
    where
        I: Iterator<Item = EEffectId> + ExactSizeIterator,
    {
        self.data.extend_entries(item_id, effects);
    }
    pub(in crate::ss::svc) fn effects_stopped<'a>(
        &mut self,
        item_id: &SsItemId,
        effects: impl Iterator<Item = &'a EEffectId>,
    ) {
        self.data.drain_entries(item_id, effects);
    }
}
