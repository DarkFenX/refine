use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, RawModifier,
        registers::standard::{
            data::StandardRegisterCtxMods,
            modifier::func::{add_cmod, remove_cmod},
        },
    },
    ud::UItem,
};

pub(super) fn proj_system_mod(
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    {
        let projectee_ship = match projectee_item {
            UItem::Ship(projectee_ship) => projectee_ship,
            _ => return None,
        };
        match rmod.affectee_filter {
            AffecteeFilter::Direct(loc) if let Ok(loc_kind) = loc.try_into() => {
                let fit_uid = projectee_ship.get_fit_uid();
                let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind);
                add_cmod(&mut reg_cmods.root, key, cmod, &mut reg_cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::Loc(loc) if let Ok(loc_kind) = loc.try_into() => {
                let fit_uid = projectee_ship.get_fit_uid();
                let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind);
                add_cmod(&mut reg_cmods.loc, key, cmod, &mut reg_cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocGrp(loc, item_grp_id) if let Ok(loc_kind) = loc.try_into() => {
                let fit_uid = projectee_ship.get_fit_uid();
                let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, item_grp_id);
                add_cmod(&mut reg_cmods.loc_grp, key, cmod, &mut reg_cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::LocSrq(loc, srq_type_id) if let Ok(loc_kind) = loc.try_into() => {
                let fit_uid = projectee_ship.get_fit_uid();
                let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                let key = (fit_uid, loc_kind, srq_type_id);
                add_cmod(&mut reg_cmods.loc_srq, key, cmod, &mut reg_cmods.by_aspec);
                Some(cmod)
            }
            AffecteeFilter::OwnSrq(srq_type_id) => {
                let fit_uid = projectee_ship.get_fit_uid();
                let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
                let key = (fit_uid, srq_type_id);
                add_cmod(&mut reg_cmods.own_srq, key, cmod, &mut reg_cmods.by_aspec);
                Some(cmod)
            }
            _ => None,
        }
    }
}
pub(super) fn unproj_system_mod(
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod: RawModifier,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    let projectee_ship = match projectee_item {
        UItem::Ship(projectee_ship) => projectee_ship,
        _ => return None,
    };
    match rmod.affectee_filter {
        AffecteeFilter::Direct(loc) if let Ok(loc_kind) = loc.try_into() => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
            let key = (fit_uid, loc_kind);
            remove_cmod(&mut reg_cmods.root, key, &cmod, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        AffecteeFilter::Loc(loc) if let Ok(loc_kind) = loc.try_into() => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
            let key = (fit_uid, loc_kind);
            remove_cmod(&mut reg_cmods.loc, key, &cmod, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        AffecteeFilter::LocGrp(loc, item_grp_id) if let Ok(loc_kind) = loc.try_into() => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
            let key = (fit_uid, loc_kind, item_grp_id);
            remove_cmod(&mut reg_cmods.loc_grp, key, &cmod, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        AffecteeFilter::LocSrq(loc, srq_type_id) if let Ok(loc_kind) = loc.try_into() => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
            let key = (fit_uid, loc_kind, srq_type_id);
            remove_cmod(&mut reg_cmods.loc_srq, key, &cmod, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        AffecteeFilter::OwnSrq(srq_type_id) => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::new_with_fit(rmod, fit_uid);
            let key = (fit_uid, srq_type_id);
            remove_cmod(&mut reg_cmods.own_srq, key, &cmod, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        _ => None,
    }
}
