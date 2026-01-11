use crate::{
    rd::RItemListId,
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, RawModifier,
        registers::standard::{
            data::{StandardRegisterCtxMods, StandardRegisterRawProjStatus},
            modifier::func::{add_cmod, remove_cmod},
        },
    },
    ud::{UItem, UItemId, UShip},
};

pub(super) fn proj_buff_mod(
    reg_proj_status: &mut StandardRegisterRawProjStatus,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_rid)) => {
            match is_item_buffable_by_proj_item_list(projectee_item, &item_list_rid) {
                true => {
                    let cmod = CtxModifier::new_with_item(rmod, projectee_uid);
                    add_cmod(&mut reg_cmods.direct, projectee_uid, cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.add_entry(projectee_uid, rmod);
                    Some(cmod)
                }
                false => {
                    reg_proj_status.inactive.add_entry(projectee_uid, rmod);
                    None
                }
            }
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_rid)) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_rid) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind);
                    add_cmod(&mut reg_cmods.loc, key, cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.add_entry(projectee_uid, rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.add_entry(projectee_uid, rmod);
                    None
                }
            }
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_rid) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    add_cmod(&mut reg_cmods.loc_grp, key, cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.add_entry(projectee_uid, rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.add_entry(projectee_uid, rmod);
                    None
                }
            }
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_rid) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid);
                    add_cmod(&mut reg_cmods.loc_srq, key, cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.add_entry(projectee_uid, rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.add_entry(projectee_uid, rmod);
                    None
                }
            }
        }
        _ => None,
    }
}
pub(super) fn unproj_buff_mod(
    reg_proj_status: &mut StandardRegisterRawProjStatus,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    // Modifiers passed to this method were not validated, so for every valid configuration we
    // have to remove a modifier from appropriate raw modifier container
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_rid)) => {
            match is_item_buffable_by_proj_item_list(projectee_item, &item_list_rid) {
                true => {
                    let cmod = CtxModifier::new_with_item(rmod, projectee_uid);
                    remove_cmod(&mut reg_cmods.direct, projectee_uid, &cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.remove_entry(projectee_uid, &rmod);
                    Some(cmod)
                }
                false => {
                    reg_proj_status.inactive.remove_entry(projectee_uid, &rmod);
                    None
                }
            }
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_rid)) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_rid) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind);
                    remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.remove_entry(projectee_uid, &rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.remove_entry(projectee_uid, &rmod);
                    None
                }
            }
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_rid) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.remove_entry(projectee_uid, &rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.remove_entry(projectee_uid, &rmod);
                    None
                }
            }
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_rid) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid);
                    remove_cmod(&mut reg_cmods.loc_srq, key, &cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.remove_entry(projectee_uid, &rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.remove_entry(projectee_uid, &rmod);
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
            if is_item_buffable_by_proj_item_list(projectee_item, &item_list_rid) =>
        {
            let cmod = CtxModifier::new_with_item(rmod, projectee_uid);
            Some(cmod)
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_rid))
        | AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), _)
        | AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), _)
            if let Some(projectee_ship) = is_ship_buffable_by_proj_item_list(projectee_item, &item_list_rid) =>
        {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, projectee_uid);
            Some(cmod)
        }
        _ => None,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Functions which are called when already projectee item is loaded/unloaded. Only modifiers which
// depend on projectee item properties should be processed by those functions.
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn load_affectee_for_proj_buff(
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_rid))
            if let Some(proj_buff_item_lists) = projectee_item.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_rid) =>
        {
            let cmod = CtxModifier::new_with_item(*rmod, projectee_uid);
            add_cmod(&mut reg_cmods.direct, projectee_uid, cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_rid))
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_rid)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
            let key = (fit_uid, loc_kind);
            add_cmod(&mut reg_cmods.loc, key, cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_rid)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
            let key = (fit_uid, loc_kind, item_grp_id);
            add_cmod(&mut reg_cmods.loc_grp, key, cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_rid)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
            let key = (fit_uid, loc_kind, srq_type_aid);
            add_cmod(&mut reg_cmods.loc_srq, key, cmod, &mut reg_cmods.by_aspec);
            true
        }
        _ => false,
    }
}
pub(super) fn unload_affectee_for_proj_buff(
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_rid))
            if let Some(proj_buff_item_lists) = projectee_item.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_rid) =>
        {
            let cmod = CtxModifier::new_with_item(*rmod, projectee_uid);
            remove_cmod(&mut reg_cmods.direct, projectee_uid, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_rid))
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_rid)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
            let key = (fit_uid, loc_kind);
            remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_rid), item_grp_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_rid)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
            let key = (fit_uid, loc_kind, item_grp_id);
            remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_rid), srq_type_aid)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_rid)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
            let key = (fit_uid, loc_kind, srq_type_aid);
            remove_cmod(&mut reg_cmods.loc_srq, key, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        _ => false,
    }
}

fn is_item_buffable_by_proj_item_list(u_item: &UItem, item_list_rid: &RItemListId) -> bool {
    match u_item.get_proj_buff_item_lists() {
        Some(buff_item_lists) => buff_item_lists.contains(item_list_rid),
        None => false,
    }
}

fn is_ship_buffable_by_proj_item_list<'a>(u_item: &'a UItem, item_list_rid: &RItemListId) -> Option<&'a UShip> {
    match u_item {
        UItem::Ship(ship) => match ship.get_proj_buff_item_lists() {
            Some(buff_item_lists) => match buff_item_lists.contains(item_list_rid) {
                true => Some(ship),
                false => None,
            },
            None => None,
        },
        _ => None,
    }
}
