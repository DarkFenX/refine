use crate::{ud::UItemKey, util::RMapRSet};

/// Projector-to-projectee relations are tracked on projector items. This tracker is for projectee-
/// to-projector relations, to allow finding projectors given a projectee.
#[derive(Clone)]
pub(in crate::sol) struct RevProjs {
    pub(super) data: RMapRSet<UItemKey, UItemKey>,
}
impl RevProjs {
    pub(in crate::sol) fn new() -> Self {
        Self { data: RMapRSet::new() }
    }
    pub(in crate::sol) fn reg_projectee(&mut self, projector_key: UItemKey, projectee_key: UItemKey) {
        self.data.add_entry(projectee_key, projector_key)
    }
    pub(in crate::sol) fn unreg_projectee(&mut self, projector_key: &UItemKey, projectee_key: &UItemKey) {
        self.data.remove_entry(projectee_key, projector_key);
    }
    pub(in crate::sol) fn iter_projectors(&self, projectee_key: &UItemKey) -> impl ExactSizeIterator<Item = UItemKey> {
        self.data.get(projectee_key).copied()
    }
}
