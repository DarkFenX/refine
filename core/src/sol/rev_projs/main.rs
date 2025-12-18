use crate::{ud::UItemKey, util::RMapRSet};

/// Projector-to-projectee relations are tracked on projector items. This tracker is for projectee-
/// to-projector relations, to allow finding projectors given a projectee.
#[derive(Clone)]
pub(crate) struct RevProjs {
    pub(super) data: RMapRSet<UItemKey, UItemKey>,
}
impl RevProjs {
    pub(in crate::sol) fn new() -> Self {
        Self { data: RMapRSet::new() }
    }
    pub(crate) fn reg_projectee(&mut self, projector_key: UItemKey, projectee_key: UItemKey) {
        self.data.add_entry(projectee_key, projector_key)
    }
    pub(crate) fn unreg_projectee(&mut self, projector_key: &UItemKey, projectee_key: UItemKey) {
        self.data.remove_entry(projectee_key, projector_key);
    }
    pub(crate) fn iter_projectors(&self, projectee_key: &UItemKey) -> impl ExactSizeIterator<Item = UItemKey> {
        self.data.get(projectee_key).copied()
    }
}
