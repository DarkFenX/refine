use crate::{rd::REffectKey, ud::UItemKey, util::RMap};

#[derive(Clone)]
pub(crate) struct Autocharges {
    data: RMap<REffectKey, UItemKey>,
}
impl Autocharges {
    pub(in crate::ud::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    // Query methods
    pub(crate) fn get_ac_key(&self, effect_key: &REffectKey) -> Option<UItemKey> {
        self.data.get(effect_key).copied()
    }
    pub(crate) fn contains_ac_for_effect(&self, effect_key: &REffectKey) -> bool {
        self.data.contains_key(effect_key)
    }
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = UItemKey> {
        self.data.values().copied()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(crate) fn set(&mut self, effect_id: REffectKey, autocharge_key: UItemKey) {
        self.data.insert(effect_id, autocharge_key);
    }
    pub(crate) fn clear(&mut self) {
        // Autocharges are supposed to be rarely used, so deallocate whenever map is empty.
        self.data = RMap::new();
    }
}
