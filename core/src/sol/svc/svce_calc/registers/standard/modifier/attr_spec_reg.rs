use crate::{
    sol::svc::svce_calc::{SolAttrSpec, SolContext, SolCtxModifier},
    util::StMapSetL1,
};

pub(super) fn reg_cmod(storage: &mut StMapSetL1<SolAttrSpec, SolCtxModifier>, modifier: SolCtxModifier) {
    if let Some(affector_attr_id) = modifier.raw.get_affector_attr_id() {
        let affector_spec = SolAttrSpec::new(modifier.raw.affector_item_id, affector_attr_id);
        storage.add_entry(affector_spec, modifier);
    }
    if let (Some(resist_attr_id), SolContext::Item(ctx_item_id)) = (modifier.raw.resist_attr_id, modifier.ctx) {
        let affector_spec = SolAttrSpec::new(ctx_item_id, resist_attr_id);
        storage.add_entry(affector_spec, modifier);
    }
    if let Some(optimal_attr_id) = modifier.raw.optimal_attr_id {
        let affector_spec = SolAttrSpec::new(modifier.raw.affector_item_id, optimal_attr_id);
        storage.add_entry(affector_spec, modifier);
    }
    if let Some(falloff_attr_id) = modifier.raw.falloff_attr_id {
        let affector_spec = SolAttrSpec::new(modifier.raw.affector_item_id, falloff_attr_id);
        storage.add_entry(affector_spec, modifier);
    }
}
pub(super) fn unreg_cmod(storage: &mut StMapSetL1<SolAttrSpec, SolCtxModifier>, modifier: &SolCtxModifier) {
    if let Some(affector_attr_id) = modifier.raw.get_affector_attr_id() {
        let affector_spec = SolAttrSpec::new(modifier.raw.affector_item_id, affector_attr_id);
        storage.remove_entry(&affector_spec, modifier);
    }
    if let (Some(resist_attr_id), SolContext::Item(ctx_item_id)) = (modifier.raw.resist_attr_id, modifier.ctx) {
        let affector_spec = SolAttrSpec::new(ctx_item_id, resist_attr_id);
        storage.remove_entry(&affector_spec, modifier);
    }
    if let Some(optimal_attr_id) = modifier.raw.optimal_attr_id {
        let affector_spec = SolAttrSpec::new(modifier.raw.affector_item_id, optimal_attr_id);
        storage.remove_entry(&affector_spec, modifier);
    }
    if let Some(falloff_attr_id) = modifier.raw.falloff_attr_id {
        let affector_spec = SolAttrSpec::new(modifier.raw.affector_item_id, falloff_attr_id);
        storage.remove_entry(&affector_spec, modifier);
    }
}
