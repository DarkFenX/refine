use crate::{
    rd::{RAttrId, REffectId},
    ud::{UData, UFitId, UItemId},
};

#[derive(thiserror::Error, Debug)]
#[error("debug error")]
pub(crate) struct DebugError {}

pub(crate) type DebugResult = Result<(), DebugError>;

pub(crate) fn check_fit_id(data: &UData, fit_id: UFitId) -> DebugResult {
    if data.fits.try_get(fit_id).is_none() {
        return Err(DebugError {});
    }
    Ok(())
}

pub(crate) fn check_item_id(data: &UData, item_id: UItemId, check_load: bool) -> DebugResult {
    let item = match data.items.try_get(item_id) {
        Some(item) => item,
        None => return Err(DebugError {}),
    };
    if check_load && !item.is_loaded() {
        return Err(DebugError {});
    }
    Ok(())
}

pub(crate) fn check_attr_id(data: &UData, attr_id: RAttrId) -> DebugResult {
    // Will crash if attr key is not valid
    data.src.get_attr_by_rid(attr_id);
    Ok(())
}

pub(crate) fn check_effect_id(data: &UData, effect_id: REffectId) -> DebugResult {
    // Will crash if effect key is not valid
    data.src.get_effect_by_rid(effect_id);
    Ok(())
}
