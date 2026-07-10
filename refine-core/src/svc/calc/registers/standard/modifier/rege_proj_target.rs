use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, Location, RawModifier,
        registers::standard::{
            data::{StandardRegisterCtxMods, StandardRegisterRawProjStatus},
            modifier::func::{add_cmod, remove_cmod},
        },
    },
    ud::{UItem, UItemId, UShipKind},
};

pub(super) fn proj_target_mod(
    reg_proj_status: &mut StandardRegisterRawProjStatus,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::Target) => {
            let cmod = CtxModifier::new_with_item(rmod, projectee_uid);
            add_cmod(&mut reg_cmods.direct, projectee_uid, cmod, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        AffecteeFilter::Loc(Location::Target) if let UItem::Ship(projectee_ship) = projectee_item => {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
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
        AffecteeFilter::LocGrp(Location::Target, item_grp_id) if let UItem::Ship(projectee_ship) = projectee_item => {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
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
        AffecteeFilter::LocSrq(Location::Target, srq_type_aid) if let UItem::Ship(projectee_ship) = projectee_item => {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
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
        AffecteeFilter::OwnSrq(srq_type_aid) if let UItem::Ship(projectee_ship) = projectee_item => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, projectee_uid);
            let key = (fit_uid, srq_type_aid);
            add_cmod(&mut reg_cmods.own_srq, key, cmod, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        _ => None,
    }
}
pub(super) fn unproj_target_mod(
    reg_proj_status: &mut StandardRegisterRawProjStatus,
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    match rmod.affectee_filter {
        AffecteeFilter::Direct(Location::Target) => {
            let cmod = CtxModifier::new_with_item(rmod, projectee_uid);
            remove_cmod(&mut reg_cmods.direct, projectee_uid, &cmod, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        AffecteeFilter::Loc(Location::Target) if let UItem::Ship(projectee_ship) = projectee_item => {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
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
        AffecteeFilter::LocGrp(Location::Target, item_grp_id) if let UItem::Ship(projectee_ship) = projectee_item => {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
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
        AffecteeFilter::LocSrq(Location::Target, srq_type_aid) if let UItem::Ship(projectee_ship) = projectee_item => {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
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
        AffecteeFilter::OwnSrq(srq_type_aid) if let UItem::Ship(projectee_ship) = projectee_item => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, projectee_uid);
            let key = (fit_uid, srq_type_aid);
            remove_cmod(&mut reg_cmods.own_srq, key, &cmod, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        _ => None,
    }
}

pub(super) fn query_target_mod(
    rmod: RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    {
        match rmod.affectee_filter {
            AffecteeFilter::Direct(Location::Target) => {
                let cmod = CtxModifier::new_with_item(rmod, projectee_uid);
                Some(cmod)
            }
            AffecteeFilter::Loc(Location::Target)
            | AffecteeFilter::LocGrp(Location::Target, _)
            | AffecteeFilter::LocSrq(Location::Target, _)
                if let UItem::Ship(projectee_ship) = projectee_item =>
            {
                match projectee_ship.get_kind() {
                    UShipKind::Ship | UShipKind::Structure => {
                        let fit_uid = projectee_ship.get_fit_uid();
                        let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, projectee_uid);
                        Some(cmod)
                    }
                    _ => None,
                }
            }
            AffecteeFilter::OwnSrq(_) if let UItem::Ship(projectee_ship) = projectee_item => {
                let fit_uid = projectee_ship.get_fit_uid();
                let cmod = CtxModifier::new_with_fit_item(rmod, fit_uid, projectee_uid);
                Some(cmod)
            }
            _ => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Functions which are called when already projectee item is loaded/unloaded. Only modifiers which
// depend on projectee item properties should be processed by those functions.
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn load_affectee_for_proj_target(
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Loc(loc)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind);
                    add_cmod(&mut reg_cmods.loc, key, cmod, &mut reg_cmods.by_aspec);
                    true
                }
                _ => false,
            }
        }
        AffecteeFilter::LocGrp(loc, item_grp_id)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    add_cmod(&mut reg_cmods.loc_grp, key, cmod, &mut reg_cmods.by_aspec);
                    true
                }
                _ => false,
            }
        }
        AffecteeFilter::LocSrq(loc, srq_type_aid)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid);
                    add_cmod(&mut reg_cmods.loc_srq, key, cmod, &mut reg_cmods.by_aspec);
                    true
                }
                _ => false,
            }
        }
        _ => false,
    }
}
pub(super) fn unload_affectee_for_proj_target(
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: &RawModifier,
    projectee_uid: UItemId,
    projectee_item: &UItem,
) -> bool {
    match rmod.affectee_filter {
        AffecteeFilter::Loc(loc)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind);
                    remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
                    true
                }
                _ => false,
            }
        }
        AffecteeFilter::LocGrp(loc, item_grp_id)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind, item_grp_id);
                    remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
                    true
                }
                _ => false,
            }
        }
        AffecteeFilter::LocSrq(loc, srq_type_aid)
            if let Location::Target = loc
                && let UItem::Ship(projectee_ship) = projectee_item =>
        {
            match projectee_ship.get_kind().try_into() {
                Ok(loc_kind) => {
                    let fit_uid = projectee_ship.get_fit_uid();
                    let cmod = CtxModifier::new_with_fit_item(*rmod, fit_uid, projectee_uid);
                    let key = (fit_uid, loc_kind, srq_type_aid);
                    remove_cmod(&mut reg_cmods.loc_srq, key, &cmod, &mut reg_cmods.by_aspec);
                    true
                }
                _ => false,
            }
        }
        _ => false,
    }
}
