use crate::sol::{
    debug::{DebugResult, check_a_effect_id, check_fit_key, check_item_key},
    svc::calc::{Context, CtxModifier, RawModifier},
    uad::Uad,
};

pub(in crate::sol::svc) fn check_raw_modifier(uad: &Uad, modifier: &RawModifier) -> DebugResult {
    check_item_key(uad, modifier.affector_item_key, true)?;
    check_a_effect_id(uad, &modifier.a_effect_id)?;
    Ok(())
}

pub(in crate::sol::svc) fn check_ctx_modifier(uad: &Uad, modifier: &CtxModifier) -> DebugResult {
    match modifier.ctx {
        // Item modifier is applied to is not necessarily loaded (e.g. a module projected to an
        // non-loaded ship)
        Context::Item(item_key) => check_item_key(uad, item_key, false)?,
        Context::Fit(fit_key) => check_fit_key(uad, fit_key)?,
        Context::None => (),
    }
    check_raw_modifier(uad, &modifier.raw)?;
    Ok(())
}
