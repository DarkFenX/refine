use crate::{
    defs::{EAttrId, SolItemId},
    sol::svc::svce_calc::SolAttrSpec,
    util::{StMapSetL1, StMapSetL2},
};

// Intended to hold direct dependencies between attributes, which are not covered by regular
// modifiers
pub(in crate::sol::svc::svce_calc) struct SolDependencyRegister {
    pub(super) data: StMapSetL1<SolAttrSpec, SolAttrSpec>,
    pub(super) item_affector_map: StMapSetL1<SolItemId, SolAttrSpec>,
    pub(super) item_affectee_map: StMapSetL2<SolItemId, SolAttrSpec, SolAttrSpec>,
}
impl SolDependencyRegister {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            data: StMapSetL1::new(),
            item_affector_map: StMapSetL1::new(),
            item_affectee_map: StMapSetL2::new(),
        }
    }
    // Query methods
    pub(in crate::sol::svc::svce_calc) fn get_tgt_attr_specs(
        &self,
        affector_item_id: &SolItemId,
        affector_attr_id: &EAttrId,
    ) -> impl ExactSizeIterator<Item = &SolAttrSpec> {
        let affector_attr_spec = SolAttrSpec::new(*affector_item_id, *affector_attr_id);
        self.data.get(&affector_attr_spec)
    }
    // Modification methods
    pub(in crate::sol::svc::svce_calc) fn add_dependency(
        &mut self,
        affector_item_id: SolItemId,
        affector_attr_id: EAttrId,
        affectee_item_id: SolItemId,
        affectee_attr_id: EAttrId,
    ) {
        let affector_attr_spec = SolAttrSpec::new(affector_item_id, affector_attr_id);
        let affectee_attr_spec = SolAttrSpec::new(affectee_item_id, affectee_attr_id);
        self.data.add_entry(affector_attr_spec, affectee_attr_spec);
        self.item_affector_map.add_entry(affector_item_id, affector_attr_spec);
        self.item_affectee_map
            .add_entry(affectee_item_id, affector_attr_spec, affectee_attr_spec);
    }
    pub(in crate::sol::svc::svce_calc) fn remove_dependency(
        &mut self,
        affector_item_id: &SolItemId,
        affector_attr_id: &EAttrId,
        affectee_item_id: &SolItemId,
        affectee_attr_id: &EAttrId,
    ) {
        let affector_attr_spec = SolAttrSpec::new(*affector_item_id, *affector_attr_id);
        let affectee_attr_spec = SolAttrSpec::new(*affectee_item_id, *affectee_attr_id);
        self.data.add_entry(affector_attr_spec, affectee_attr_spec);
        self.item_affector_map
            .remove_entry(affector_item_id, &affector_attr_spec);
        self.item_affectee_map
            .remove_entry(affectee_item_id, &affector_attr_spec, &affectee_attr_spec);
    }
    pub(in crate::sol::svc::svce_calc) fn clear_item_data(&mut self, item_id: &SolItemId) {
        // Remove data where item is affector
        if let Some(affector_attr_specs) = self.item_affector_map.remove_key(item_id) {
            for affector_attr_spec in affector_attr_specs {
                self.data.remove_key(&affector_attr_spec);
            }
        }
        // Remove data where item is affectee
        if let Some(attr_spec_map) = self.item_affectee_map.remove_l1(item_id) {
            for (affector_attr_spec, affectee_attr_specs) in attr_spec_map.iter() {
                for affectee_attr_spec in affectee_attr_specs {
                    self.data.remove_entry(affector_attr_spec, affectee_attr_spec);
                }
            }
        }
    }
}
