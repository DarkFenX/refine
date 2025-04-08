use crate::sol::{
    debug::{DebugResult, check_a_effect_id, check_fit_id, check_item_key},
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
        Context::Item(item_key) => check_item_key(uad, item_key, true)?,
        Context::Fit(fit_id) => check_fit_id(uad, &fit_id)?,
        Context::None => (),
    }
    check_raw_modifier(uad, &modifier.raw)?;
    Ok(())
}
