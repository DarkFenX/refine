use crate::{
    ad,
    uad::{Uad, UadFitKey, UadItemKey},
};

#[derive(thiserror::Error, Debug)]
#[error("debug error")]
pub(crate) struct DebugError {}

pub(crate) type DebugResult = Result<(), DebugError>;

pub(crate) fn check_fit_key(uad: &Uad, fit_key: UadFitKey) -> DebugResult {
    if uad.fits.try_get(fit_key).is_none() {
        return Err(DebugError {});
    }
    Ok(())
}

pub(crate) fn check_item_key(uad: &Uad, item_key: UadItemKey, check_load: bool) -> DebugResult {
    let item = match uad.items.try_get(item_key) {
        Some(item) => item,
        None => return Err(DebugError {}),
    };
    if check_load && !item.is_loaded() {
        return Err(DebugError {});
    }
    Ok(())
}

pub(crate) fn check_a_effect_id(uad: &Uad, a_effect_id: &ad::AEffectId) -> DebugResult {
    if uad.src.get_r_effect(a_effect_id).is_none() {
        return Err(DebugError {});
    }
    Ok(())
}
