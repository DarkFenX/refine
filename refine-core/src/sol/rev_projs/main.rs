use crate::{ud::UItemId, util::RMapRSet};

/// Projector-to-projectee relations are tracked on projector items. This tracker is for projectee-
/// to-projector relations, to allow finding projectors given a projectee.
#[derive(Clone)]
pub(crate) struct RevProjs {
    pub(super) data: RMapRSet<UItemId, UItemId>,
}
impl RevProjs {
    pub(in crate::sol) fn new() -> Self {
        Self { data: RMapRSet::new() }
    }
    pub(crate) fn reg_projectee(&mut self, projector_uid: UItemId, projectee_uid: UItemId) {
        self.data.add_entry(projectee_uid, projector_uid)
    }
    pub(crate) fn unreg_projectee(&mut self, projector_uid: &UItemId, projectee_uid: UItemId) {
        self.data.remove_entry(projectee_uid, projector_uid);
    }
    pub(crate) fn iter_projectors(&self, projectee_uid: &UItemId) -> impl ExactSizeIterator<Item = UItemId> {
        self.data.get(projectee_uid).copied()
    }
}
