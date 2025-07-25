use crate::{
    ad,
    ud::{UData, UFitKey, UItemKey},
};

#[derive(thiserror::Error, Debug)]
#[error("debug error")]
pub(crate) struct DebugError {}

pub(crate) type DebugResult = Result<(), DebugError>;

pub(crate) fn check_fit_key(u_data: &UData, fit_key: UFitKey) -> DebugResult {
    if u_data.fits.try_get(fit_key).is_none() {
        return Err(DebugError {});
    }
    Ok(())
}

pub(crate) fn check_item_key(u_data: &UData, item_key: UItemKey, check_load: bool) -> DebugResult {
    let item = match u_data.items.try_get(item_key) {
        Some(item) => item,
        None => return Err(DebugError {}),
    };
    if check_load && !item.is_loaded() {
        return Err(DebugError {});
    }
    Ok(())
}

pub(crate) fn check_a_effect_id(u_data: &UData, a_effect_id: &ad::AEffectId) -> DebugResult {
    if u_data.src.get_r_effect(a_effect_id).is_none() {
        return Err(DebugError {});
    }
    Ok(())
}
