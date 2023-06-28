use std::collections::HashSet;

use crate::{
    defs::{EAttrId, SsItemId},
    util::KeyedStorage2L,
};

pub(in crate::ss::svc::calc) struct AttrCapData {
    data: KeyedStorage2L<SsItemId, EAttrId, EAttrId>,
}
impl AttrCapData {
    pub(in crate::ss::svc::calc) fn new() -> Self {
        Self {
            data: KeyedStorage2L::new(),
        }
    }
    // Getters
    pub(in crate::ss::svc::calc) fn get_capped_attr_ids(
        &mut self,
        item_id: &SsItemId,
        capping_attr_id: &EAttrId,
    ) -> Option<&HashSet<EAttrId>> {
        self.data.get_l2(item_id, capping_attr_id)
    }
    // Maintenance
    pub(in crate::ss::svc::calc) fn add_cap(
        &mut self,
        item_id: SsItemId,
        capping_attr_id: EAttrId,
        capped_attr_id: EAttrId,
    ) {
        self.data.add(item_id, capping_attr_id, capped_attr_id)
    }
    pub(in crate::ss::svc::calc) fn clear_item_caps(&mut self, item_id: &SsItemId) {
        self.data.remove_l1(item_id)
    }
}
