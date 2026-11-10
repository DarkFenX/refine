use crate::{
    svc::calc::{
        AffecteeFilter, CtxModifier, RawModId, RawModifier,
        registers::{
            RegOp,
            standard::{data::StandardRegisterCtxMods, modifier::func::process_cmod},
        },
    },
    ud::UItem,
};

pub(super) fn process_system_mod_projection<const OP: RegOp>(
    reg_cmods: &mut StandardRegisterCtxMods,
    rmod_id: RawModId,
    rmod: RawModifier,
    projectee_item: &UItem,
) -> Option<CtxModifier> {
    let UItem::Ship(projectee_ship) = projectee_item else {
        return None;
    };
    match rmod.affectee_filter {
        AffecteeFilter::Direct(loc) if let Ok(loc_kind) = loc.try_into() => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::with_fit(rmod, fit_uid);
            let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
            process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.root, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        AffecteeFilter::Loc(loc) if let Ok(loc_kind) = loc.try_into() => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::with_fit(rmod, fit_uid);
            let key = (fit_uid, loc_kind, cmod.rmod.affectee_attr_rid);
            process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        AffecteeFilter::LocGrp(loc, item_grp_id) if let Ok(loc_kind) = loc.try_into() => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::with_fit(rmod, fit_uid);
            let key = (fit_uid, loc_kind, item_grp_id, cmod.rmod.affectee_attr_rid);
            process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc_grp, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        AffecteeFilter::LocSrq(loc, srq_type_aid) if let Ok(loc_kind) = loc.try_into() => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::with_fit(rmod, fit_uid);
            let key = (fit_uid, loc_kind, srq_type_aid, cmod.rmod.affectee_attr_rid);
            process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.loc_srq, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        AffecteeFilter::OwnSrq(srq_type_aid) => {
            let fit_uid = projectee_ship.get_fit_uid();
            let cmod = CtxModifier::with_fit(rmod, fit_uid);
            let key = (fit_uid, srq_type_aid, cmod.rmod.affectee_attr_rid);
            process_cmod::<OP, _>(key, rmod_id, &cmod, &mut reg_cmods.own_srq, &mut reg_cmods.by_aspec);
            Some(cmod)
        }
        _ => None,
    }
}
