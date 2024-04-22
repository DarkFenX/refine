use crate::{
    defs::{EAttrId, EEffectId, SolFitId, SolItemId},
    sol::SolView,
    util::{DebugError, DebugResult},
};

pub(in crate::sol::svc) fn check_fit(sol_view: &SolView, fit_id: &SolFitId) -> DebugResult {
    if sol_view.fits.get_fit(fit_id).is_err() {
        return Err(DebugError::new());
    }
    Ok(())
}

pub(in crate::sol::svc) fn check_item(sol_view: &SolView, item_id: &SolItemId) -> DebugResult {
    let item = match sol_view.items.get_item(item_id) {
        Ok(item) => item,
        _ => return Err(DebugError::new()),
    };
    if item.get_a_item().is_err() {
        return Err(DebugError::new());
    }
    Ok(())
}

pub(in crate::sol::svc) fn check_effect(sol_view: &SolView, effect_id: &EEffectId) -> DebugResult {
    if sol_view.src.get_a_effect(effect_id).is_none() {
        return Err(DebugError::new());
    }
    Ok(())
}

pub(in crate::sol::svc) fn check_attr(sol_view: &SolView, attr_id: &EAttrId) -> DebugResult {
    if sol_view.src.get_a_attr(attr_id).is_none() {
        return Err(DebugError::new());
    }
    Ok(())
}
