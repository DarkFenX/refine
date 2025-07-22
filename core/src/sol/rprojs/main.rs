use crate::{uad::UadItemKey, util::RMapRSet};

/// Projector-to-projectee relations are tracked on projector items. This tracker is for projectee-
/// to-projector relations, to allow finding projectors given a projectee.
#[derive(Clone)]
pub(in crate::sol) struct RProjs {
    pub(super) data: RMapRSet<UadItemKey, UadItemKey>,
}
impl RProjs {
    pub(in crate::sol) fn new() -> Self {
        Self { data: RMapRSet::new() }
    }
    pub(in crate::sol) fn reg_projectee(&mut self, projector_key: UadItemKey, projectee_key: UadItemKey) {
        self.data.add_entry(projectee_key, projector_key)
    }
    pub(in crate::sol) fn unreg_projectee(&mut self, projector_key: &UadItemKey, projectee_key: &UadItemKey) {
        self.data.remove_entry(projectee_key, projector_key);
    }
    pub(in crate::sol) fn iter_projectors(
        &self,
        projectee_key: &UadItemKey,
    ) -> impl ExactSizeIterator<Item = &UadItemKey> {
        self.data.get(projectee_key)
    }
}
