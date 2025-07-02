use crate::{
    dbg::{DebugResult, check_a_effect_id, check_fit_key, check_item_key},
    svc::calc::{Context, CtxModifier, RawModifier},
    uad::Uad,
};

pub(in crate::svc) fn check_rmod(uad: &Uad, rmod: &RawModifier) -> DebugResult {
    check_item_key(uad, rmod.affector_espec.item_key, true)?;
    check_a_effect_id(uad, &rmod.affector_espec.a_effect_id)?;
    Ok(())
}

pub(in crate::svc) fn check_cmod(uad: &Uad, cmod: &CtxModifier) -> DebugResult {
    match cmod.ctx {
        // Item modifier is applied to is not necessarily loaded (e.g. a module projected to a
        // non-loaded ship)
        Context::Item(item_key) => check_item_key(uad, item_key, false)?,
        Context::Fit(fit_key) => check_fit_key(uad, fit_key)?,
        Context::None => (),
    }
    check_rmod(uad, &cmod.raw)?;
    Ok(())
}
