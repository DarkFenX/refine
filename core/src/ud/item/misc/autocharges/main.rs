use crate::{rd::REffectId, ud::UItemId, util::RMap};

#[derive(Clone)]
pub(crate) struct UAutocharges {
    data: RMap<REffectId, UItemId>,
}
impl UAutocharges {
    pub(in crate::ud::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    // Query methods
    pub(crate) fn get_ac_uid(&self, effect_rid: &REffectId) -> Option<UItemId> {
        self.data.get(effect_rid).copied()
    }
    pub(crate) fn contains_ac_for_effect(&self, effect_rid: &REffectId) -> bool {
        self.data.contains_key(effect_rid)
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = UItemId> {
        self.data.values().copied()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(crate) fn set(&mut self, effect_rid: REffectId, autocharge_uid: UItemId) {
        self.data.insert(effect_rid, autocharge_uid);
    }
    pub(crate) fn clear(&mut self) {
        // Autocharges are supposed to be rarely used, so deallocate whenever map is empty.
        self.data = RMap::new();
    }
}
