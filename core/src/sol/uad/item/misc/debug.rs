use crate::{
    defs::{SolFitId, SolItemId},
    sol::{uad::SolUad, SolDebugError, SolDebugResult},
};

pub(in crate::sol::uad::item) fn check_item(uad: &SolUad, item_id: &SolItemId) -> SolDebugResult {
    if uad.items.get_item(item_id).is_err() {
        return Err(SolDebugError::new());
    }
    Ok(())
}

pub(in crate::sol::uad::item) fn check_fit(uad: &SolUad, fit_id: &SolFitId) -> SolDebugResult {
    if uad.fits.get_fit(fit_id).is_err() {
        return Err(SolDebugError::new());
    }
    Ok(())
}
