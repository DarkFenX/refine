use crate::{sol::ItemId, util::RMapRSet};

#[derive(Clone)]
pub(in crate::sol) struct ProjTracker {
    pub(super) data: RMapRSet<ItemId, ItemId>,
}
impl ProjTracker {
    pub(in crate::sol) fn new() -> Self {
        Self { data: RMapRSet::new() }
    }
    pub(in crate::sol) fn reg_projectee(&mut self, projector_item_id: ItemId, projectee_item_id: ItemId) {
        self.data.add_entry(projectee_item_id, projector_item_id)
    }
    pub(in crate::sol) fn unreg_projectee(&mut self, projector_item_id: &ItemId, projectee_item_id: &ItemId) {
        self.data.remove_entry(projectee_item_id, projector_item_id);
    }
    pub(in crate::sol) fn iter_projectors(&self, projectee_item_id: &ItemId) -> impl ExactSizeIterator<Item = &ItemId> {
        self.data.get(projectee_item_id)
    }
}
