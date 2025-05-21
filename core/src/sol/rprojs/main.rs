use crate::{sol::ItemKey, util::RMapRSet};

/// Projector-to-projectee relations are tracked on projector items. This tracker is for projectee-
/// to-projector relations, to allow finding projectors given a projectee.
#[derive(Clone)]
pub(in crate::sol) struct RProjs {
    pub(super) data: RMapRSet<ItemKey, ItemKey>,
}
impl RProjs {
    pub(in crate::sol) fn new() -> Self {
        Self { data: RMapRSet::new() }
    }
    pub(in crate::sol) fn reg_projectee(&mut self, projector_item_key: ItemKey, projectee_item_key: ItemKey) {
        self.data.add_entry(projectee_item_key, projector_item_key)
    }
    pub(in crate::sol) fn unreg_projectee(&mut self, projector_item_key: &ItemKey, projectee_item_key: &ItemKey) {
        self.data.remove_entry(projectee_item_key, projector_item_key);
    }
    pub(in crate::sol) fn iter_projectors(
        &self,
        projectee_item_key: &ItemKey,
    ) -> impl ExactSizeIterator<Item = &ItemKey> {
        self.data.get(projectee_item_key)
    }
}
