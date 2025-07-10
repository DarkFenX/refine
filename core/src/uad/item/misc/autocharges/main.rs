use crate::{ad, def::ItemKey, util::RMap};

#[derive(Clone)]
pub(crate) struct Autocharges {
    data: RMap<ad::AEffectId, ItemKey>,
}
impl Autocharges {
    pub(in crate::uad::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    // Query methods
    pub(crate) fn values(&self) -> impl ExactSizeIterator<Item = &ItemKey> {
        self.data.values()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub(crate) fn contains_ac_for_effect(&self, a_effect_id: &ad::AEffectId) -> bool {
        self.data.contains_key(a_effect_id)
    }
    // Modification methods
    pub(crate) fn set(&mut self, a_effect_id: ad::AEffectId, autocharge_key: ItemKey) {
        self.data.insert(a_effect_id, autocharge_key);
    }
    pub(crate) fn clear(&mut self) {
        // Autocharges are supposed to be rarely used, so deallocate whenever map is empty.
        self.data = RMap::new();
    }
}
