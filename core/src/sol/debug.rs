use crate::{
    defs::{EAttrId, EEffectId, SolFitId, SolItemId},
    sol::{uad::SolUad, SolDebugError, SolDebugResult},
};

pub(in crate::sol) fn check_fit(uad: &SolUad, fit_id: &SolFitId) -> SolDebugResult {
    if uad.fits.get_fit(fit_id).is_err() {
        return Err(SolDebugError::new());
    }
    Ok(())
}

pub(in crate::sol) fn check_item(uad: &SolUad, item_id: &SolItemId, check_load: bool) -> SolDebugResult {
    let item = match uad.items.get_item(item_id) {
        Ok(item) => item,
        _ => return Err(SolDebugError::new()),
    };
    if check_load && !item.is_loaded() {
        return Err(SolDebugError::new());
    }
    Ok(())
}

pub(in crate::sol) fn check_effect(uad: &SolUad, effect_id: &EEffectId) -> SolDebugResult {
    if uad.src.get_a_effect(effect_id).is_none() {
        return Err(SolDebugError::new());
    }
    Ok(())
}

pub(in crate::sol) fn check_attr(uad: &SolUad, attr_id: &EAttrId) -> SolDebugResult {
    if uad.src.get_a_attr(attr_id).is_none() {
        return Err(SolDebugError::new());
    }
    Ok(())
}
