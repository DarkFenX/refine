use crate::sol::{
    debug::{check_effect, check_fit, check_item, SolDebugResult},
    svc::calc::{SolContext, SolCtxModifier, SolRawModifier},
    uad::SolUad,
};

pub(in crate::sol::svc) fn check_raw_modifier(uad: &SolUad, modifier: &SolRawModifier) -> SolDebugResult {
    check_item(uad, &modifier.affector_item_id, true)?;
    check_effect(uad, &modifier.effect_id)?;
    Ok(())
}

pub(in crate::sol::svc) fn check_ctx_modifier(uad: &SolUad, modifier: &SolCtxModifier) -> SolDebugResult {
    match modifier.ctx {
        SolContext::Item(item_id) => check_item(uad, &item_id, true)?,
        SolContext::Fit(fit_id) => check_fit(uad, &fit_id)?,
        SolContext::None => (),
    }
    check_raw_modifier(uad, &modifier.raw)?;
    Ok(())
}
