use crate::{defs::SsItemId, util::KeyedStorage1L};

pub(in crate::ss) struct TgtTracker {
    data: KeyedStorage1L<SsItemId, SsItemId>,
}
impl TgtTracker {
    pub(in crate::ss) fn new() -> Self {
        Self {
            data: KeyedStorage1L::new(),
        }
    }
    pub(in crate::ss) fn reg_tgt(&mut self, src_item_id: SsItemId, tgt_item_id: SsItemId) {
        self.data.add_entry(tgt_item_id, src_item_id)
    }
    pub(in crate::ss) fn unreg_tgt(&mut self, src_item_id: &SsItemId, tgt_item_id: &SsItemId) {
        self.data.remove_entry(tgt_item_id, src_item_id)
    }
    pub(in crate::ss) fn iter_srcs(&self, tgt_item_id: &SsItemId) -> impl ExactSizeIterator<Item = &SsItemId> {
        self.data.get(tgt_item_id)
    }
}
