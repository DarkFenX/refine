use std::collections::HashSet;

use crate::{
    defs::{EffectId, SsItemId},
    util::KeyedStorage1L,
};

pub(in crate::ss::svc) struct RunningEffects {
    data: KeyedStorage1L<SsItemId, EffectId>,
}
impl RunningEffects {
    pub(in crate::ss::svc) fn new() -> Self {
        Self {
            data: KeyedStorage1L::new(),
        }
    }
    // Getters
    pub(in crate::ss::svc) fn get_running_effects(&self, item_id: &SsItemId) -> Option<&HashSet<EffectId>> {
        self.data.get(item_id)
    }
    pub(in crate::ss::svc) fn is_effect_running(&self, item_id: &SsItemId, effect_id: &EffectId) -> bool {
        match self.data.get(item_id) {
            Some(effect_ids) => effect_ids.contains(effect_id),
            None => false,
        }
    }
    // Maintenance methods
    pub(in crate::ss::svc) fn effects_started<I>(&mut self, item_id: SsItemId, effects: I)
    where
        I: Iterator<Item = EffectId> + ExactSizeIterator,
    {
        self.data.extend(item_id, effects);
    }
    pub(in crate::ss::svc) fn effects_stopped<I>(&mut self, item_id: &SsItemId, effects: I)
    where
        I: Iterator<Item = EffectId>,
    {
        self.data.drain(item_id, effects);
    }
}
