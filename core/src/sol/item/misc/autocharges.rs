use crate::{
    defs::{EEffectId, SolItemId},
    util::StMap,
};

pub(in crate::sol) struct SolAutocharges {
    data: StMap<EEffectId, SolItemId>,
}
impl SolAutocharges {
    pub(in crate::sol::item) fn new() -> Self {
        Self { data: StMap::new() }
    }
    // Query methods
    pub(in crate::sol) fn get(&self, effect_id: &EEffectId) -> Option<&SolItemId> {
        self.data.get(effect_id)
    }
    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = (&EEffectId, &SolItemId)> {
        self.data.iter()
    }
    pub(in crate::sol) fn values(&self) -> impl ExactSizeIterator<Item = &SolItemId> {
        self.data.values()
    }
    // Modification methods
    pub(in crate::sol) fn set(&mut self, effect_id: EEffectId, autocharge_item_id: SolItemId) {
        self.data.insert(effect_id, autocharge_item_id);
    }
    pub(in crate::sol) fn clear(&mut self) {
        self.data.clear();
        // Autocharges are supposed to be rarely used, so deallocate whenever map is empty.
        self.data.shrink_to_fit();
    }
}
