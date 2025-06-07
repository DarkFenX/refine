use crate::{ad, sol::ItemKey, util::RMap};

#[derive(Clone)]
pub(in crate::sol) struct Autocharges {
    data: RMap<ad::AEffectId, ItemKey>,
}
impl Autocharges {
    pub(in crate::sol::uad::item) fn new() -> Self {
        Self { data: RMap::new() }
    }
    // Query methods
    pub(in crate::sol) fn values(&self) -> impl ExactSizeIterator<Item = &ItemKey> {
        self.data.values()
    }
    pub(in crate::sol) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    // Modification methods
    pub(in crate::sol) fn set(&mut self, a_effect_id: ad::AEffectId, autocharge_key: ItemKey) {
        self.data.insert(a_effect_id, autocharge_key);
    }
    pub(in crate::sol) fn clear(&mut self) {
        // Autocharges are supposed to be rarely used, so deallocate whenever map is empty.
        self.data = RMap::new();
    }
}
