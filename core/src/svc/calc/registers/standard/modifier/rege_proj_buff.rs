use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, RawModifier,
        registers::standard::{
            data::{StandardRegisterCtxMods, StandardRegisterRawProjStatus},
            func::{add_cmod, remove_cmod},
        },
    },
    ud::{UItem, UItemKey},
};

pub(super) fn proj_buff_mod(
    reg_proj_status: &mut StandardRegisterRawProjStatus,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    projectee_key: UItemKey,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
            match projectee_item.is_item_buffable_by_item_list(&item_list_id) {
                true => {
                    let cmod = CtxModifier::new_with_projectee_item(rmod, projectee_key);
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
        AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
            match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
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
        AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
            match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
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
        AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
            match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
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
        AffecteeFilter::Direct(Location::ItemList(item_list_id)) => {
            match projectee_item.is_item_buffable_by_item_list(&item_list_id) {
                true => {
                    let cmod = CtxModifier::new_with_projectee_item(rmod, projectee_key);
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
        AffecteeFilter::Loc(Location::ItemList(item_list_id)) => {
            match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
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
        AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id) => {
            match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
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
        AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id) => {
            match projectee_item.is_ship_buffable_by_item_list(&item_list_id) {
                Some(projectee_ship) if let Ok(loc_kind) = projectee_ship.get_kind().try_into() => {
                    let fit_key = projectee_ship.get_fit_key();
                    let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
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
        AffecteeFilter::Direct(Location::ItemList(item_list_id))
            if projectee_item.is_item_buffable_by_item_list(&item_list_id) =>
        {
            let cmod = CtxModifier::new_with_projectee_item(rmod, projectee_key);
            Some(cmod)
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_id))
        | AffecteeFilter::LocGrp(Location::ItemList(item_list_id), _)
        | AffecteeFilter::LocSrq(Location::ItemList(item_list_id), _)
            if let Some(projectee_ship) = projectee_item.is_ship_buffable_by_item_list(&item_list_id) =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(rmod, fit_key, projectee_key);
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
        AffecteeFilter::Direct(Location::ItemList(item_list_id))
            if let Some(buffable_item_lists) = projectee_item.get_item_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let cmod = CtxModifier::new_with_projectee_item(*rmod, projectee_key);
            add_cmod(&mut reg_cmods.direct, projectee_key, cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_id))
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, loc_kind);
            add_cmod(&mut reg_cmods.loc, key, cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, loc_kind, item_grp_id);
            add_cmod(&mut reg_cmods.loc_grp, key, cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
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
        AffecteeFilter::Direct(Location::ItemList(item_list_id))
            if let Some(buffable_item_lists) = projectee_item.get_item_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id) =>
        {
            let cmod = CtxModifier::new_with_projectee_item(*rmod, projectee_key);
            remove_cmod(&mut reg_cmods.direct, projectee_key, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::Loc(Location::ItemList(item_list_id))
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, loc_kind);
            remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocGrp(Location::ItemList(item_list_id), item_grp_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, loc_kind, item_grp_id);
            remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        AffecteeFilter::LocSrq(Location::ItemList(item_list_id), srq_type_id)
            if let UItem::Ship(projectee_ship) = projectee_item
                && let Some(buffable_item_lists) = projectee_ship.get_buff_item_lists()
                && buffable_item_lists.contains(&item_list_id)
                && let Ok(loc_kind) = projectee_ship.get_kind().try_into() =>
        {
            let fit_key = projectee_ship.get_fit_key();
            let cmod = CtxModifier::new_with_projectee_fit_item(*rmod, fit_key, projectee_key);
            let key = (fit_key, loc_kind, srq_type_id);
            remove_cmod(&mut reg_cmods.loc_srq, key, &cmod, &mut reg_cmods.by_aspec);
            true
        }
        _ => false,
    }
}
