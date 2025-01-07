use std::hash::Hash;

use crate::{
    sol::svc::calc::{SolAttrSpec, SolContext, SolCtxModifier},
    util::StMapSetL1,
};

pub(super) fn add_ctx_modifier<K: Eq + Hash>(
    main_storage: &mut StMapSetL1<K, SolCtxModifier>,
    key: K,
    ctx_modifier: SolCtxModifier,
    attr_spec_storage: &mut StMapSetL1<SolAttrSpec, SolCtxModifier>,
) {
    main_storage.add_entry(key, ctx_modifier);
    if let Some(affector_attr_id) = ctx_modifier.raw.get_affector_attr_id() {
        let affector_spec = SolAttrSpec::new(ctx_modifier.raw.affector_item_id, affector_attr_id);
        attr_spec_storage.add_entry(affector_spec, ctx_modifier);
    }
    if let (Some(resist_attr_id), SolContext::Item(ctx_item_id)) = (ctx_modifier.raw.resist_attr_id, ctx_modifier.ctx) {
        let affector_spec = SolAttrSpec::new(ctx_item_id, resist_attr_id);
        attr_spec_storage.add_entry(affector_spec, ctx_modifier);
    }
    if let Some(optimal_attr_id) = ctx_modifier.raw.optimal_attr_id {
        let affector_spec = SolAttrSpec::new(ctx_modifier.raw.affector_item_id, optimal_attr_id);
        attr_spec_storage.add_entry(affector_spec, ctx_modifier);
    }
    if let Some(falloff_attr_id) = ctx_modifier.raw.falloff_attr_id {
        let affector_spec = SolAttrSpec::new(ctx_modifier.raw.affector_item_id, falloff_attr_id);
        attr_spec_storage.add_entry(affector_spec, ctx_modifier);
    }
}

pub(super) fn remove_ctx_modifier<K: Eq + Hash>(
    main_storage: &mut StMapSetL1<K, SolCtxModifier>,
    key: &K,
    ctx_modifier: &SolCtxModifier,
    attr_spec_storage: &mut StMapSetL1<SolAttrSpec, SolCtxModifier>,
) {
    main_storage.remove_entry(key, ctx_modifier);
    if let Some(affector_attr_id) = ctx_modifier.raw.get_affector_attr_id() {
        let affector_spec = SolAttrSpec::new(ctx_modifier.raw.affector_item_id, affector_attr_id);
        attr_spec_storage.remove_entry(&affector_spec, ctx_modifier);
    }
    if let (Some(resist_attr_id), SolContext::Item(ctx_item_id)) = (ctx_modifier.raw.resist_attr_id, ctx_modifier.ctx) {
        let affector_spec = SolAttrSpec::new(ctx_item_id, resist_attr_id);
        attr_spec_storage.remove_entry(&affector_spec, ctx_modifier);
    }
    if let Some(optimal_attr_id) = ctx_modifier.raw.optimal_attr_id {
        let affector_spec = SolAttrSpec::new(ctx_modifier.raw.affector_item_id, optimal_attr_id);
        attr_spec_storage.remove_entry(&affector_spec, ctx_modifier);
    }
    if let Some(falloff_attr_id) = ctx_modifier.raw.falloff_attr_id {
        let affector_spec = SolAttrSpec::new(ctx_modifier.raw.affector_item_id, falloff_attr_id);
        attr_spec_storage.remove_entry(&affector_spec, ctx_modifier);
    }
}
