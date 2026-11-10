use crate::{
    rd::RItemListId,
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, RawModId, RawModifier,
        registers::{
            RegOp, process_entry,
            standard::{
                data::{StandardRegisterCtxMods, StandardRegisterRawProjStatus},
                modifier::func::process_cmod,
            },
        },
    },
    ud::{UItem, UItemId, UShip},
};

pub(super) fn process_buff_mod_projection<const OP: RegOp>(
    reg_proj_status: &mut StandardRegisterRawProjStatus,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod_id: RawModId,
    rmod: RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_rid)) => {
            match is_item_buffable_by_proj_item_list(projectee_item, item_list_rid) {
                true => {
                    let cmod = CtxModifier::with_item(rmod, projectee_uid);
                    let key = (projectee_uid, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.direct, &mut reg_cmods.by_aspec);
                    process_entry::<OP, _, _>(&mut reg_proj_status.active, projectee_uid, rmod_id);
                    Some(cmod)
                }
                false => {
                    process_entry::<OP, _, _>(&mut reg_proj_status.inactive, projectee_uid, rmod_id);
                    None
                }
            }
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_rid)) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, item_list_rid) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_ship_kind().try_into() => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc, &mut reg_cmods.by_aspec);
                    process_entry::<OP, _, _>(&mut reg_proj_status.active, projectee_uid, rmod_id);
                    Some(cmod)
                }
                _ => {
                    process_entry::<OP, _, _>(&mut reg_proj_status.inactive, projectee_uid, rmod_id);
                    None
                }
            }
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, item_list_rid) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_ship_kind().try_into() => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc_grp, &mut reg_cmods.by_aspec);
                    process_entry::<OP, _, _>(&mut reg_proj_status.active, projectee_uid, rmod_id);
                    Some(cmod)
                }
                _ => {
                    process_entry::<OP, _, _>(&mut reg_proj_status.inactive, projectee_uid, rmod_id);
                    None
                }
            }
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, item_list_rid) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_ship_kind().try_into() => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::with_fit_item(rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
                    process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc_srq, &mut reg_cmods.by_aspec);
                    process_entry::<OP, _, _>(&mut reg_proj_status.active, projectee_uid, rmod_id);
                    Some(cmod)
                }
                _ => {
                    process_entry::<OP, _, _>(&mut reg_proj_status.inactive, projectee_uid, rmod_id);
                    None
                }
            }
        }
        _ => None,
    }
}
pub(super) fn query_buff_mod(rmod: RawModifier, projectee_uid: UItemId, projectee_item: &UItem) -> Option<CtxModifier> {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_rid))
            if is_item_buffable_by_proj_item_list(projectee_item, item_list_rid) =>
        {
            let cmod = CtxModifier::with_item(rmod, projectee_uid);
            Some(cmod)
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_rid))
        | AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), _)
        | AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), _)
            if let Some(projectee_ship) = is_ship_buffable_by_proj_item_list(projectee_item, item_list_rid) =>
        {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::with_fit_item(rmod, fit_uid, projectee_uid);
            Some(cmod)
        }
        _ => None,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Functions which are called when already projectee item is loaded/unloaded. Only modifiers which
// depend on projectee item properties should be processed by those functions.
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn process_affectee_for_proj_buff<const OP: RegOp>(
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod_id: RawModId,
    rmod: &RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_rid))
            if let Some(proj_buff_item_lists) = projectee_item.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_rid) =>
        {
            let cmod = CtxModifier::with_item(*rmod, projectee_uid);
            let key = (projectee_uid, cmod.rmod.affectee_attr_rid);
            process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.direct, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_rid))
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(projectee_rib) = projectee_ship.get_r_item_base()
                && projectee_rib.proj_buff_item_list_rids.contains(&item_list_rid)
                && let Ok(loc_kind) = projectee_ship.get_ship_kind().try_into() =>
        {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::with_fit_item(*rmod, fit_uid, projectee_uid);
            let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
            process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(projectee_rib) = projectee_ship.get_r_item_base()
                && projectee_rib.proj_buff_item_list_rids.contains(&item_list_rid)
                && let Ok(loc_kind) = projectee_ship.get_ship_kind().try_into() =>
        {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::with_fit_item(*rmod, fit_uid, projectee_uid);
            let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
            process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc_grp, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(projectee_rib) = projectee_ship.get_r_item_base()
                && projectee_rib.proj_buff_item_list_rids.contains(&item_list_rid)
                && let Ok(loc_kind) = projectee_ship.get_ship_kind().try_into() =>
        {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::with_fit_item(*rmod, fit_uid, projectee_uid);
            let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
            process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc_srq, &mut reg_cmods.by_aspec);
            true
        }
        _ => false,
    }
}
fn is_item_buffable_by_proj_item_list(u_item: &UItem, item_list_rid: RItemListId) -> bool {
    match u_item.get_proj_buff_item_lists() {
        Some(buff_item_lists) => buff_item_lists.contains(&item_list_rid),
        None => false,
    }
}

fn is_ship_buffable_by_proj_item_list(u_item: &UItem, item_list_rid: RItemListId) -> Option<&UShip> {
    match u_item {
        UItem::Ship(ship) => match ship.get_r_item_base() {
            Some(ship_rib) => match ship_rib.proj_buff_item_list_rids.contains(&item_list_rid) {
                true => Some(ship),
                false => None,
            },
            None => None,
        },
        _ => None,
    }
}
