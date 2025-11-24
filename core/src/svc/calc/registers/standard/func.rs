use std::hash::{BuildHasher, Hash};

use crate::{
    ad::AItemListId,
    misc::AttrSpec,
    svc::{SvcCtx, calc::CtxModifier},
    ud::{UFitKey, UItemKey},
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
    if let Some(affector_attr_id) = cmod.raw.get_affector_attr_id() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, affector_attr_id);
        aspec_storage.add_entry(affector_aspec, cmod);
    }
    if let (Some(resist_attr_id), Some(ctx_item_id)) = (cmod.raw.resist_attr_id, cmod.ctx.get_projectee_key()) {
        let affector_aspec = AttrSpec::new(ctx_item_id, resist_attr_id);
        aspec_storage.add_entry(affector_aspec, cmod);
    }
    for proj_attr_id in cmod.raw.proj_attr_ids.into_iter().flatten() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, proj_attr_id);
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
    if let Some(affector_attr_id) = cmod.raw.get_affector_attr_id() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, affector_attr_id);
        aspec_storage.remove_entry(affector_aspec, cmod);
    }
    if let (Some(resist_attr_id), Some(ctx_item_id)) = (cmod.raw.resist_attr_id, cmod.ctx.get_projectee_key()) {
        let affector_aspec = AttrSpec::new(ctx_item_id, resist_attr_id);
        aspec_storage.remove_entry(affector_aspec, cmod);
    }
    for proj_attr_id in cmod.raw.proj_attr_ids.into_iter().flatten() {
        let affector_aspec = AttrSpec::new(cmod.raw.affector_espec.item_key, proj_attr_id);
        aspec_storage.remove_entry(affector_aspec, cmod);
    }
}

// TODO: look for a way to optimize it by moving ship info to fit, or something in the register
pub(super) fn is_fit_ship_on_item_list(ctx: SvcCtx, fit_key: UFitKey, item_list_id: &AItemListId) -> Option<UItemKey> {
    let fit = ctx.u_data.fits.get(fit_key);
    let ship_key = fit.ship?;
    let ship = ctx.u_data.items.get(ship_key);
    match ship.get_item_buff_item_lists()?.contains(item_list_id) {
        true => Some(ship_key),
        false => None,
    }
}
