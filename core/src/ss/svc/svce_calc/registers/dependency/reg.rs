use crate::{util::KeyedStorage2L, EAttrId, SsItemId};
use std::collections::HashSet;

use super::attr_spec::AttrSpec;

// Intended to hold direct dependencies between attributes, which are not covered by regular
// modifiers
pub(in crate::ss::svc::svce_calc) struct DependencyRegister {
    data: KeyedStorage2L<SsItemId, EAttrId, AttrSpec>,
}
impl DependencyRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            data: KeyedStorage2L::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc::svce_calc) fn get_tgt_attr_specs(
        &self,
        src_item_id: &SsItemId,
        src_attr_id: &EAttrId,
    ) -> Option<&HashSet<AttrSpec>> {
        self.data.get_l2(src_item_id, src_attr_id)
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn add_dependency(
        &mut self,
        src_item_id: SsItemId,
        src_attr_id: EAttrId,
        tgt_item_id: SsItemId,
        tgt_attr_id: EAttrId,
    ) {
        let tgt_attr_spec = AttrSpec::new(tgt_item_id, tgt_attr_id);
        self.data.add(src_item_id, src_attr_id, tgt_attr_spec);
    }
    pub(in crate::ss::svc::svce_calc) fn clear_src_item_data(&mut self, item_id: &SsItemId) {
        self.data.remove_l1(item_id)
    }
}
