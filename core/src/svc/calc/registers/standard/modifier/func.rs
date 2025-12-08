use std::hash::{BuildHasher, Hash};

use crate::{
    misc::AttrSpec,
    svc::calc::CtxModifier,
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
    if let Some(affector_attr_key) = cmod.raw.get_affector_attr_key() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, affector_attr_key);
        aspec_storage.add_entry(affector_aspec, cmod);
    }
    if let (Some(resist_attr_key), Some(ctx_item_key)) = (cmod.raw.resist_attr_key, cmod.ctx.get_item_key()) {
        let affector_aspec = AttrSpec::new(ctx_item_key, resist_attr_key);
        aspec_storage.add_entry(affector_aspec, cmod);
    }
    for proj_attr_key in cmod.raw.proj_attr_keys.into_iter().flatten() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, proj_attr_key);
        aspec_storage.add_entry(affector_aspec, cmod);
    }
}

pub(super) fn remove_cmod<K, H1, H2>(
    main_storage: &mut MapSet<K, CtxModifier, H1, H2>,
    key: K,
    cmod: &CtxModifier,
    aspec_storage: &mut RMapRSet<AttrSpec, CtxModifier>,
) where
    K: Eq + Hash,
    H1: BuildHasher + Default,
    H2: BuildHasher + Default,
{
    main_storage.remove_entry(key, cmod);
    if let Some(affector_attr_key) = cmod.raw.get_affector_attr_key() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, affector_attr_key);
        aspec_storage.remove_entry(affector_aspec, cmod);
    }
    if let (Some(resist_attr_key), Some(ctx_item_key)) = (cmod.raw.resist_attr_key, cmod.ctx.get_item_key()) {
        let affector_aspec = AttrSpec::new(ctx_item_key, resist_attr_key);
        aspec_storage.remove_entry(affector_aspec, cmod);
    }
    for proj_attr_key in cmod.raw.proj_attr_keys.into_iter().flatten() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, proj_attr_key);
        aspec_storage.remove_entry(affector_aspec, cmod);
    }
}
