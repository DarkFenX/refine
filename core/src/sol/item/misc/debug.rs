use crate::{
    defs::{SolFitId, SolItemId},
    sol::{SolDebugError, SolDebugResult, SolView},
};

pub(in crate::sol::item) fn check_item(sol_view: &SolView, item_id: &SolItemId) -> SolDebugResult {
    match sol_view.items.get_item(item_id) {
        Ok(_) => Ok(()),
        _ => return Err(SolDebugError::new()),
    }
}

pub(in crate::sol::item) fn check_fit(sol_view: &SolView, fit_id: &SolFitId) -> SolDebugResult {
    if sol_view.fits.get_fit(fit_id).is_err() {
        return Err(SolDebugError::new());
    }
    Ok(())
}
