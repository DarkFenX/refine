use crate::{
    defs::{EAttrId, SolItemId},
    sol::svc::svce_calc::SolAttrSpec,
    util::{StMapSetL1, StMapSetL2},
};

// Intended to hold direct dependencies between attributes, which are not covered by regular
// modifiers
pub(in crate::sol::svc::svce_calc) struct SolDependencyRegister {
    pub(super) data: StMapSetL1<SolAttrSpec, SolAttrSpec>,
    pub(super) item_src_map: StMapSetL1<SolItemId, SolAttrSpec>,
    pub(super) item_tgt_map: StMapSetL2<SolItemId, SolAttrSpec, SolAttrSpec>,
}
impl SolDependencyRegister {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            data: StMapSetL1::new(),
            item_src_map: StMapSetL1::new(),
            item_tgt_map: StMapSetL2::new(),
        }
    }
    // Query methods
    pub(in crate::sol::svc::svce_calc) fn get_tgt_attr_specs(
        &self,
        src_item_id: &SolItemId,
        src_attr_id: &EAttrId,
    ) -> impl ExactSizeIterator<Item = &SolAttrSpec> {
        let src_attr_spec = SolAttrSpec::new(*src_item_id, *src_attr_id);
        self.data.get(&src_attr_spec)
    }
    // Modification methods
    pub(in crate::sol::svc::svce_calc) fn add_dependency(
        &mut self,
        src_item_id: SolItemId,
        src_attr_id: EAttrId,
        tgt_item_id: SolItemId,
        tgt_attr_id: EAttrId,
    ) {
        let src_attr_spec = SolAttrSpec::new(src_item_id, src_attr_id);
        let tgt_attr_spec = SolAttrSpec::new(tgt_item_id, tgt_attr_id);
        self.data.add_entry(src_attr_spec, tgt_attr_spec);
        self.item_src_map.add_entry(src_item_id, src_attr_spec);
        self.item_tgt_map.add_entry(tgt_item_id, src_attr_spec, tgt_attr_spec);
    }
    pub(in crate::sol::svc::svce_calc) fn remove_dependency(
        &mut self,
        src_item_id: &SolItemId,
        src_attr_id: &EAttrId,
        tgt_item_id: &SolItemId,
        tgt_attr_id: &EAttrId,
    ) {
        let src_attr_spec = SolAttrSpec::new(*src_item_id, *src_attr_id);
        let tgt_attr_spec = SolAttrSpec::new(*tgt_item_id, *tgt_attr_id);
        self.data.add_entry(src_attr_spec, tgt_attr_spec);
        self.item_src_map.remove_entry(src_item_id, &src_attr_spec);
        self.item_tgt_map
            .remove_entry(tgt_item_id, &src_attr_spec, &tgt_attr_spec);
    }
    pub(in crate::sol::svc::svce_calc) fn clear_item_data(&mut self, item_id: &SolItemId) {
        // Remove data where item is source of dependency
        if let Some(attr_specs) = self.item_src_map.remove_key(item_id) {
            for attr_spec in attr_specs {
                self.data.remove_key(&attr_spec);
            }
        }
        // Remove data where item is target of dependency
        if let Some(attr_spec_map) = self.item_tgt_map.remove_l1(item_id) {
            for (src_attr_spec, tgt_attr_specs) in attr_spec_map.iter() {
                for tgt_attr_spec in tgt_attr_specs {
                    self.data.remove_entry(src_attr_spec, tgt_attr_spec);
                }
            }
        }
    }
}
