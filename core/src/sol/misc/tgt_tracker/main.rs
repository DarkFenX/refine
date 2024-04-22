use crate::{defs::SolItemId, util::StMapSetL1};

pub(in crate::sol) struct SolTgtTracker {
    pub(super) data: StMapSetL1<SolItemId, SolItemId>,
}
impl SolTgtTracker {
    pub(in crate::sol) fn new() -> Self {
        Self {
            data: StMapSetL1::new(),
        }
    }
    pub(in crate::sol) fn reg_tgt(&mut self, src_item_id: SolItemId, tgt_item_id: SolItemId) {
        self.data.add_entry(tgt_item_id, src_item_id)
    }
    pub(in crate::sol) fn unreg_tgt(&mut self, src_item_id: &SolItemId, tgt_item_id: &SolItemId) {
        self.data.remove_entry(tgt_item_id, src_item_id)
    }
    pub(in crate::sol) fn iter_srcs(&self, tgt_item_id: &SolItemId) -> impl ExactSizeIterator<Item = &SolItemId> {
        self.data.get(tgt_item_id)
    }
}
