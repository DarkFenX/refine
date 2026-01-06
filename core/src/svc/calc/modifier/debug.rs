use crate::{
    dbg::{DebugResult, check_attr_rid, check_effect_rid, check_fit_uid, check_item_uid},
    svc::calc::{Context, CtxModifier, RawModifier},
    ud::UData,
};

pub(in crate::svc) fn check_rmod(u_data: &UData, rmod: &RawModifier) -> DebugResult {
    check_item_uid(u_data, rmod.affector_espec.item_uid, true)?;
    check_effect_rid(u_data, rmod.affector_espec.effect_rid)?;
    check_attr_rid(u_data, rmod.affectee_attr_rid)?;
    if let Some(attr_rid) = rmod.buff_type_attr_rid {
        check_attr_rid(u_data, attr_rid)?;
    }
    for &attr_rid in rmod.proj_attr_rids.iter() {
        if let Some(attr_rid) = attr_rid {
            check_attr_rid(u_data, attr_rid)?;
        }
    }
    if let Some(attr_rid) = rmod.resist_attr_rid {
        check_attr_rid(u_data, attr_rid)?;
    }
    Ok(())
}

pub(in crate::svc) fn check_cmod(u_data: &UData, cmod: &CtxModifier) -> DebugResult {
    match cmod.ctx {
        // Item modifier is applied to is not necessarily loaded (e.g. a module projected to a
        // non-loaded ship)
        Context::None => (),
        Context::Item(item_uid) => check_item_uid(u_data, item_uid, false)?,
        Context::Fit(fit_uid) => check_fit_uid(u_data, fit_uid)?,
        Context::FitItem(fit_uid, item_uid) => {
            check_fit_uid(u_data, fit_uid)?;
            check_item_uid(u_data, item_uid, false)?;
        }
    }
    check_rmod(u_data, &cmod.raw)?;
    Ok(())
}
