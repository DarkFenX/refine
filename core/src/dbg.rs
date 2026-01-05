use crate::{
    rd::{RAttrId, REffectId},
    ud::{UData, UFitId, UItemId},
};

#[derive(thiserror::Error, Debug)]
#[error("debug error")]
pub(crate) struct DebugError {}

pub(crate) type DebugResult = Result<(), DebugError>;

pub(crate) fn check_fit_uid(data: &UData, fit_uid: UFitId) -> DebugResult {
    if data.fits.try_get(fit_uid).is_none() {
        return Err(DebugError {});
    }
    Ok(())
}

pub(crate) fn check_item_uid(data: &UData, item_uid: UItemId, check_load: bool) -> DebugResult {
    let item = match data.items.try_get(item_uid) {
        Some(item) => item,
        None => return Err(DebugError {}),
    };
    if check_load && !item.is_loaded() {
        return Err(DebugError {});
    }
    Ok(())
}

pub(crate) fn check_attr_rid(data: &UData, attr_rid: RAttrId) -> DebugResult {
    // Will crash if attr ID is not valid
    data.src.get_attr_by_rid(attr_rid);
    Ok(())
}

pub(crate) fn check_effect_rid(data: &UData, effect_rid: REffectId) -> DebugResult {
    // Will crash if effect ID is not valid
    data.src.get_effect_by_rid(effect_rid);
    Ok(())
}
