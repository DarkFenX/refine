use crate::{
    rd::RItemListKey,
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, RawModifier,
        registers::standard::{
            data::{StandardRegisterCtxMods, StandardRegisterRawProjStatus},
            modifier::func::{add_cmod, remove_cmod},
        },
    },
    ud::{UItem, UItemKey, UShip},
};

pub(super) fn proj_buff_mod(
    reg_proj_status: &mut StandardRegisterRawProjStatus,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_key)) => {
            match is_item_buffable_by_proj_item_list(projectee_item, &item_list_key) {
                true => {
                    let cmod = CtxModifier::new_with_item(rmod, projectee_key);
                    add_cmod(&mut reg_cmods.direct, projectee_key, cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.add_entry(projectee_key, rmod);
                    Some(cmod)
                }
                false => {
                    reg_proj_status.inactive.add_entry(projectee_key, rmod);
                    None
                }
            }
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_key)) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_key) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind);
                    add_cmod(&mut reg_cmods.loc, key, cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.add_entry(projectee_key, rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.add_entry(projectee_key, rmod);
                    None
                }
            }
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_key), item_grp_id) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_key) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind, item_grp_id);
                    add_cmod(&mut reg_cmods.loc_grp, key, cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.add_entry(projectee_key, rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.add_entry(projectee_key, rmod);
                    None
                }
            }
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_key), srq_type_id) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_key) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind, srq_type_id);
                    add_cmod(&mut reg_cmods.loc_srq, key, cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.add_entry(projectee_key, rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.add_entry(projectee_key, rmod);
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
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    // Modifiers passed to this method were not validated, so for every valid configuration we
    // have to remove a modifier from appropriate raw modifier container
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_key)) => {
            match is_item_buffable_by_proj_item_list(projectee_item, &item_list_key) {
                true => {
                    let cmod = CtxModifier::new_with_item(rmod, projectee_key);
                    remove_cmod(&mut reg_cmods.direct, projectee_key, &cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.remove_entry(projectee_key, &rmod);
                    Some(cmod)
                }
                false => {
                    reg_proj_status.inactive.remove_entry(projectee_key, &rmod);
                    None
                }
            }
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_key)) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_key) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind);
                    remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.remove_entry(projectee_key, &rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.remove_entry(projectee_key, &rmod);
                    None
                }
            }
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_key), item_grp_id) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_key) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind, item_grp_id);
                    remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.remove_entry(projectee_key, &rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.remove_entry(projectee_key, &rmod);
                    None
                }
            }
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_key), srq_type_id) => {
            match is_ship_buffable_by_proj_item_list(projectee_item, &item_list_key) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, projectee_key);
                    let key = (fit_key, loc_kind, srq_type_id);
                    remove_cmod(&mut reg_cmods.loc_srq, key, &cmod, &mut reg_cmods.by_aspec);
                    reg_proj_status.active.remove_entry(projectee_key, &rmod);
                    Some(cmod)
                }
                _ => {
                    reg_proj_status.inactive.remove_entry(projectee_key, &rmod);
                    None
                }
            }
        }
        _ => None,
    }
}

pub(super) fn query_buff_mod(
    rmod: RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_key))
            if is_item_buffable_by_proj_item_list(projectee_item, &item_list_key) =>
        {
            let cmod = CtxModifier::new_with_item(rmod, projectee_key);
            Some(cmod)
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_key))
        | AffecteeFilter::LocGrp(Location::ItemList(item_list_key), _)
        | AffecteeFilter::LocSrq(Location::ItemList(item_list_key), _)
            if let Some(projectee_ship) = is_ship_buffable_by_proj_item_list(projectee_item, &item_list_key) =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_fit_item(rmod, fit_key, projectee_key);
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
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_key))
            if let Some(proj_buff_item_lists) = projectee_item.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_key) =>
        {
            let cmod = CtxModifier::new_with_item(*rmod, projectee_key);
            add_cmod(&mut reg_cmods.direct, projectee_key, cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_key))
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_key)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, loc_kind);
            add_cmod(&mut reg_cmods.loc, key, cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_key), item_grp_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_key)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, loc_kind, item_grp_id);
            add_cmod(&mut reg_cmods.loc_grp, key, cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_key), srq_type_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_key)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, loc_kind, srq_type_id);
            add_cmod(&mut reg_cmods.loc_srq, key, cmod, &mut reg_cmods.by_aspec);
            true
        }
        _ => false,
    }
}
pub(super) fn unload_affectee_for_proj_buff(
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_key))
            if let Some(proj_buff_item_lists) = projectee_item.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_key) =>
        {
            let cmod = CtxModifier::new_with_item(*rmod, projectee_key);
            remove_cmod(&mut reg_cmods.direct, projectee_key, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_key))
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_key)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, loc_kind);
            remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_key), item_grp_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_key)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, loc_kind, item_grp_id);
            remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_key), srq_type_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(proj_buff_item_lists) = projectee_ship.get_proj_buff_item_lists()
                && proj_buff_item_lists.contains(&item_list_key)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, loc_kind, srq_type_id);
            remove_cmod(&mut reg_cmods.loc_srq, key, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        _ => false,
    }
}

fn is_item_buffable_by_proj_item_list(u_item: &UItem, item_list_key: &RItemListKey) -> bool {
    match u_item.get_proj_buff_item_lists() {
        Some(buff_item_lists) => buff_item_lists.contains(item_list_key),
        None => false,
    }
}

fn is_ship_buffable_by_proj_item_list<'a>(u_item: &'a UItem, item_list_key: &RItemListKey) -> Option<&'a UShip> {
    match u_item {
        UItem::Ship(ship) => match ship.get_proj_buff_item_lists() {
            Some(buff_item_lists) => match buff_item_lists.contains(item_list_key) {
                true => Some(ship),
                false => None,
            },
            None => None,
        },
        _ => None,
    }
}
