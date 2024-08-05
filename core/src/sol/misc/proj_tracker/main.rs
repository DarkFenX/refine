use crate::{defs::SolItemId, util::StMapSetL1};

#[derive(Clone)]
pub(in crate::sol) struct SolProjTracker {
    pub(super) data: StMapSetL1<SolItemId, SolItemId>,
}
impl SolProjTracker {
    pub(in crate::sol) fn new() -> Self {
        Self {
            data: StMapSetL1::new(),
        }
    }
    pub(in crate::sol) fn reg_projectee(&mut self, projector_item_id: SolItemId, projectee_item_id: SolItemId) {
        self.data.add_entry(projectee_item_id, projector_item_id)
    }
    pub(in crate::sol) fn unreg_projectee(&mut self, projector_item_id: &SolItemId, projectee_item_id: &SolItemId) {
        self.data.remove_entry(projectee_item_id, projector_item_id)
    }
    pub(in crate::sol) fn iter_projectors(
        &self,
        projectee_item_id: &SolItemId,
    ) -> impl ExactSizeIterator<Item = &SolItemId> {
        self.data.get(projectee_item_id)
    }
}
