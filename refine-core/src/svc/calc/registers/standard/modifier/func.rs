use std::hash::Hash;

use crate::{
    misc::AttrSpec,
    svc::calc::{CtxModKey, CtxModifier, ModifierKind, RawModId, registers::RegOp},
    util::RMapRSet,
};

pub(super) fn process_cmod<const OP: RegOp, K>(
    key: K,
    rmod_id: RawModId,
    cmod: &CtxModifier,
    main_storage: &mut RMapRSet<K, CtxModKey>,
    aspec_storage: &mut RMapRSet<AttrSpec, CtxModKey>,
) where
    K: Eq + Hash,
{
    match OP {
        RegOp::Reg => add_cmod(key, rmod_id, cmod, main_storage, aspec_storage),
        RegOp::Unreg => remove_cmod(key, rmod_id, cmod, main_storage, aspec_storage),
    }
}

pub(super) fn add_cmod<K>(
    key: K,
    rmod_id: RawModId,
    cmod: &CtxModifier,
    main_storage: &mut RMapRSet<K, CtxModKey>,
    aspec_storage: &mut RMapRSet<AttrSpec, CtxModKey>,
) where
    K: Eq + Hash,
{
    let ckey = cmod.get_ckey(rmod_id);
    main_storage.add_entry(key, ckey);
    // Local template modifiers are handled separately
    if let Some(affector_attr_rid) = cmod.rmod.get_affector_attr_rid()
        && !(rmod_id.is_template() && matches!(cmod.rmod.kind, ModifierKind::Local))
    {
        let affector_aspec = AttrSpec::new(cmod.rmod.affector_espec.item_uid, affector_attr_rid);
        aspec_storage.add_entry(affector_aspec, ckey);
    }
    if let (Some(resist_attr_rid), Some(ctx_item_uid)) = (cmod.rmod.resist_attr_rid, cmod.ctx.get_item_uid()) {
        let affector_aspec = AttrSpec::new(ctx_item_uid, resist_attr_rid);
        aspec_storage.add_entry(affector_aspec, ckey);
    }
    if let Some(mspec) = cmod.rmod.proj_spec {
        for proj_attr_rid in mspec.proj_attr_rids.into_iter().flatten() {
            let affector_aspec = AttrSpec::new(cmod.rmod.affector_espec.item_uid, proj_attr_rid);
            aspec_storage.add_entry(affector_aspec, ckey);
        }
    }
}

pub(super) fn remove_cmod<K>(
    key: K,
    rmod_id: RawModId,
    cmod: &CtxModifier,
    main_storage: &mut RMapRSet<K, CtxModKey>,
    aspec_storage: &mut RMapRSet<AttrSpec, CtxModKey>,
) where
    K: Eq + Hash,
{
    let ckey = cmod.get_ckey(rmod_id);
    main_storage.remove_entry(key, &ckey);
    // Local template modifiers are handled separately
    if let Some(affector_attr_rid) = cmod.rmod.get_affector_attr_rid()
        && !(rmod_id.is_template() && matches!(cmod.rmod.kind, ModifierKind::Local))
    {
        let affector_aspec = AttrSpec::new(cmod.rmod.affector_espec.item_uid, affector_attr_rid);
        aspec_storage.remove_entry(affector_aspec, &ckey);
    }
    if let (Some(resist_attr_rid), Some(ctx_item_uid)) = (cmod.rmod.resist_attr_rid, cmod.ctx.get_item_uid()) {
        let affector_aspec = AttrSpec::new(ctx_item_uid, resist_attr_rid);
        aspec_storage.remove_entry(affector_aspec, &ckey);
    }
    if let Some(mspec) = cmod.rmod.proj_spec {
        for proj_attr_rid in mspec.proj_attr_rids.into_iter().flatten() {
            let affector_aspec = AttrSpec::new(cmod.rmod.affector_espec.item_uid, proj_attr_rid);
            aspec_storage.remove_entry(affector_aspec, &ckey);
        }
    }
}
