use crate::{
    defs::{EAttrId, EEffectId, SolItemId},
    sol::svc::svce_calc::SolAttrSpec,
    util::{StMapSetL1, StMapSetL2},
};

// Intended to hold ad-hoc dependencies between attributes, which are not covered by registers
// which hold data about regular modifiers.
pub(in crate::sol::svc::svce_calc) struct SolDependencyRegister {
    // Map<affector spec, affectee specs>
    pub(super) data: StMapSetL1<SolAttrSpec, SolAttrSpec>,
    // Map<affector item ID, affector specs>
    pub(super) affector_by_item: StMapSetL1<SolItemId, SolAttrSpec>,
    // Map<affectee item ID, Map<affector spec, affectee specs>>
    pub(super) affectee_by_item: StMapSetL2<SolItemId, SolAttrSpec, SolAttrSpec>,
    // Map<(source item ID, source effect ID), (affector spec, affectee spec)>
    pub(super) by_source: StMapSetL1<(SolItemId, EEffectId), (SolAttrSpec, SolAttrSpec)>,
    // Map<item ID, (source item ID, source effect ID)>
    pub(super) source_by_item: StMapSetL1<SolItemId, (SolItemId, EEffectId)>,
}
impl SolDependencyRegister {
    pub(in crate::sol::svc::svce_calc) fn new() -> Self {
        Self {
            data: StMapSetL1::new(),
            affector_by_item: StMapSetL1::new(),
            affectee_by_item: StMapSetL2::new(),
            by_source: StMapSetL1::new(),
            source_by_item: StMapSetL1::new(),
        }
    }
    // Query methods
    pub(in crate::sol::svc::svce_calc) fn get_affectee_attr_specs(
        &self,
        affector_item_id: &SolItemId,
        affector_attr_id: &EAttrId,
    ) -> impl ExactSizeIterator<Item = &SolAttrSpec> {
        let affector_spec = SolAttrSpec::new(*affector_item_id, *affector_attr_id);
        self.data.get(&affector_spec)
    }
    // Modification methods
    pub(in crate::sol::svc::svce_calc) fn add_direct_local(
        &mut self,
        item_id: SolItemId,
        affector_attr_id: EAttrId,
        affectee_attr_id: EAttrId,
    ) {
        let affector_spec = SolAttrSpec::new(item_id, affector_attr_id);
        let affectee_spec = SolAttrSpec::new(item_id, affectee_attr_id);
        self.data.add_entry(affector_spec, affectee_spec);
        self.affector_by_item.add_entry(item_id, affector_spec);
        self.affectee_by_item.add_entry(item_id, affector_spec, affectee_spec);
    }
    pub(in crate::sol::svc::svce_calc) fn add_with_source(
        &mut self,
        source_item_id: SolItemId,
        source_effect_id: EEffectId,
        affector_item_id: SolItemId,
        affector_attr_id: EAttrId,
        affectee_item_id: SolItemId,
        affectee_attr_id: EAttrId,
    ) {
        let affector_spec = SolAttrSpec::new(affector_item_id, affector_attr_id);
        let affectee_spec = SolAttrSpec::new(affectee_item_id, affectee_attr_id);
        self.data.add_entry(affector_spec, affectee_spec);
        self.affector_by_item.add_entry(affector_item_id, affector_spec);
        self.affectee_by_item
            .add_entry(affectee_item_id, affector_spec, affectee_spec);
        self.by_source
            .add_entry((source_item_id, source_effect_id), (affector_spec, affectee_spec));
        self.source_by_item
            .add_entry(affector_item_id, (source_item_id, source_effect_id));
        self.source_by_item
            .add_entry(affectee_item_id, (source_item_id, source_effect_id));
    }
    pub(in crate::sol::svc::svce_calc) fn remove_with_source(
        &mut self,
        source_item_id: &SolItemId,
        source_effect_id: &EEffectId,
        affector_item_id: &SolItemId,
        affector_attr_id: &EAttrId,
        affectee_item_id: &SolItemId,
        affectee_attr_id: &EAttrId,
    ) {
        let affector_spec = SolAttrSpec::new(*affector_item_id, *affector_attr_id);
        let affectee_spec = SolAttrSpec::new(*affectee_item_id, *affectee_attr_id);
        self.data.remove_entry(&affector_spec, &affectee_spec);
        self.affector_by_item.remove_entry(affector_item_id, &affector_spec);
        self.affectee_by_item
            .remove_entry(affectee_item_id, &affector_spec, &affectee_spec);
        self.by_source
            .remove_entry(&(*source_item_id, *source_effect_id), &(affector_spec, affectee_spec));
        self.source_by_item
            .remove_entry(affector_item_id, &(*source_item_id, *source_effect_id));
        self.source_by_item
            .remove_entry(affectee_item_id, &(*source_item_id, *source_effect_id));
    }
    pub(in crate::sol::svc::svce_calc) fn remove_by_source(
        &mut self,
        source_item_id: &SolItemId,
        source_effect_id: &EEffectId,
    ) {
        if let Some(spec_iter) = self.by_source.remove_key(&(*source_item_id, *source_effect_id)) {
            for (affector_spec, affectee_spec) in spec_iter {
                self.data.remove_entry(&affector_spec, &affectee_spec);
                self.affector_by_item
                    .remove_entry(&affector_spec.item_id, &affector_spec);
                self.affectee_by_item
                    .remove_entry(&affectee_spec.item_id, &affector_spec, &affectee_spec);
                self.source_by_item
                    .remove_entry(&affector_spec.item_id, &(*source_item_id, *source_effect_id));
                self.source_by_item
                    .remove_entry(&affectee_spec.item_id, &(*source_item_id, *source_effect_id));
            }
        }
    }
    pub(in crate::sol::svc::svce_calc) fn remove_item(&mut self, item_id: &SolItemId) {
        // Remove data where item is affector
        if let Some(affector_specs) = self.affector_by_item.remove_key(item_id) {
            for affector_spec in affector_specs {
                self.data.remove_key(&affector_spec);
            }
        }
        // Remove data where item is affectee
        if let Some(spec_map) = self.affectee_by_item.remove_l1(item_id) {
            for (affector_spec, affectee_specs) in spec_map.iter() {
                for affectee_spec in affectee_specs {
                    self.data.remove_entry(affector_spec, affectee_spec);
                }
            }
        }
        // Remove references to sources where this item was involved
        if let Some(sources) = self.source_by_item.remove_key(item_id) {
            for source in sources {
                self.by_source.remove_key(&source);
            }
        }
    }
}
