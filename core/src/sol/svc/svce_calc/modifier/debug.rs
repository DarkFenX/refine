use crate::sol::{
    svc::{
        debug::{check_effect, check_fit, check_item},
        svce_calc::{SolContext, SolCtxModifier, SolRawModifier},
    },
    SolDebugResult, SolView,
};

pub(in crate::sol::svc) fn check_raw_modifier(sol_view: &SolView, modifier: &SolRawModifier) -> SolDebugResult {
    check_item(sol_view, &modifier.affector_item_id)?;
    check_effect(sol_view, &modifier.effect_id)?;
    Ok(())
}

pub(in crate::sol::svc) fn check_ctx_modifier(sol_view: &SolView, modifier: &SolCtxModifier) -> SolDebugResult {
    match modifier.ctx {
        SolContext::Item(item_id) => check_item(sol_view, &item_id)?,
        SolContext::Fit(fit_id) => check_fit(sol_view, &fit_id)?,
        SolContext::None => (),
    }
    check_raw_modifier(sol_view, &modifier.raw)?;
    Ok(())
}
