use crate::{
    dbg::{DebugResult, check_a_effect_id, check_fit_key, check_item_key},
    svc::calc::{Context, CtxModifier, RawModifier},
    ud::UData,
};

pub(in crate::svc) fn check_rmod(u_data: &UData, rmod: &RawModifier) -> DebugResult {
    check_item_key(u_data, rmod.affector_espec.item_key, true)?;
    check_a_effect_id(u_data, &rmod.affector_espec.a_effect_id)?;
    Ok(())
}

pub(in crate::svc) fn check_cmod(u_data: &UData, cmod: &CtxModifier) -> DebugResult {
    match cmod.ctx {
        // Item modifier is applied to is not necessarily loaded (e.g. a module projected to a
        // non-loaded ship)
        Context::Item(item_key) => check_item_key(u_data, item_key, false)?,
        Context::Fit(fit_key) => check_fit_key(u_data, fit_key)?,
        Context::None => (),
    }
    check_rmod(u_data, &cmod.raw)?;
    Ok(())
}
