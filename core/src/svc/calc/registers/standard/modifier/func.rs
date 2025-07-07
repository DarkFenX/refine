use std::hash::{BuildHasher, Hash};

use crate::{
    misc::AttrSpec,
    svc::calc::{Context, CtxModifier},
    util::{MapSet, RMapRSet},
};

pub(super) fn add_cmod<K, H1, H2>(
    main_storage: &mut MapSet<K, CtxModifier, H1, H2>,
    key: K,
    cmod: CtxModifier,
    aspec_storage: &mut RMapRSet<AttrSpec, CtxModifier>,
) where
    K: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    main_storage.add_entry(key, cmod);
    if let Some(affector_a_attr_id) = cmod.raw.get_affector_a_attr_id() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, affector_a_attr_id);
        aspec_storage.add_entry(affector_aspec, cmod);
    }
    if let (Some(resist_a_attr_id), Context::Item(ctx_item_id)) = (cmod.raw.resist_a_attr_id, cmod.ctx) {
        let affector_aspec = AttrSpec::new(ctx_item_id, resist_a_attr_id);
        aspec_storage.add_entry(affector_aspec, cmod);
    }
    for proj_a_attr_id in cmod.raw.proj_a_attr_ids.into_iter().flatten() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, proj_a_attr_id);
        aspec_storage.add_entry(affector_aspec, cmod);
    }
}

pub(super) fn remove_cmod<K, H1, H2>(
    main_storage: &mut MapSet<K, CtxModifier, H1, H2>,
    key: &K,
    cmod: &CtxModifier,
    aspec_storage: &mut RMapRSet<AttrSpec, CtxModifier>,
) where
    K: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    main_storage.remove_entry(key, cmod);
    if let Some(affector_a_attr_id) = cmod.raw.get_affector_a_attr_id() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, affector_a_attr_id);
        aspec_storage.remove_entry(&affector_aspec, cmod);
    }
    if let (Some(resist_a_attr_id), Context::Item(ctx_item_id)) = (cmod.raw.resist_a_attr_id, cmod.ctx) {
        let affector_aspec = AttrSpec::new(ctx_item_id, resist_a_attr_id);
        aspec_storage.remove_entry(&affector_aspec, cmod);
    }
    for proj_a_attr_id in cmod.raw.proj_a_attr_ids.into_iter().flatten() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, proj_a_attr_id);
        aspec_storage.remove_entry(&affector_aspec, cmod);
    }
}
