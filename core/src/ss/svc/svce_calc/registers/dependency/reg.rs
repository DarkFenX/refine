use std::collections::HashSet;

use crate::{
    defs::{EAttrId, SsItemId},
    util::{KeyedStorage1L, KeyedStorage2L},
};

use super::attr_spec::AttrSpec;

// Intended to hold direct dependencies between attributes, which are not covered by regular
// modifiers
pub(in crate::ss::svc::svce_calc) struct DependencyRegister {
    data: KeyedStorage1L<AttrSpec, AttrSpec>,
    item_src_map: KeyedStorage1L<SsItemId, AttrSpec>,
    item_tgt_map: KeyedStorage2L<SsItemId, AttrSpec, AttrSpec>,
}
impl DependencyRegister {
    pub(in crate::ss::svc::svce_calc) fn new() -> Self {
        Self {
            data: KeyedStorage1L::new(),
            item_src_map: KeyedStorage1L::new(),
            item_tgt_map: KeyedStorage2L::new(),
        }
    }
    // Query methods
    pub(in crate::ss::svc::svce_calc) fn get_tgt_attr_specs(
        &self,
        src_item_id: &SsItemId,
        src_attr_id: &EAttrId,
    ) -> Option<&HashSet<AttrSpec>> {
        let src_attr_spec = AttrSpec::new(*src_item_id, *src_attr_id);
        self.data.get(&src_attr_spec)
    }
    // Modification methods
    pub(in crate::ss::svc::svce_calc) fn add_dependency(
        &mut self,
        src_item_id: SsItemId,
        src_attr_id: EAttrId,
        tgt_item_id: SsItemId,
        tgt_attr_id: EAttrId,
    ) {
        let src_attr_spec = AttrSpec::new(src_item_id, src_attr_id);
        let tgt_attr_spec = AttrSpec::new(tgt_item_id, tgt_attr_id);
        self.data.add_entry(src_attr_spec, tgt_attr_spec);
        self.item_src_map.add_entry(src_item_id, src_attr_spec);
        self.item_tgt_map.add_entry(tgt_item_id, src_attr_spec, tgt_attr_spec);
    }
    pub(in crate::ss::svc::svce_calc) fn clear_item_data(&mut self, item_id: &SsItemId) {
        // Remove data where item is source of dependency
        if let Some(attr_specs) = self.item_src_map.remove_key(item_id) {
            for attr_spec in attr_specs.iter() {
                self.data.remove_key(attr_spec);
            }
        }
        // Remove data where item is target of dependency
        if let Some(attr_spec_map) = self.item_tgt_map.remove_l1(item_id) {
            for (src_attr_spec, tgt_attr_specs) in attr_spec_map.iter() {
                for tgt_attr_spec in tgt_attr_specs.iter() {
                    self.data.remove_entry(src_attr_spec, tgt_attr_spec);
                }
            }
        }
    }
}
