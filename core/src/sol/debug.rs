use crate::{
    ad,
    sol::{FitKey, ItemKey, uad::Uad},
};

#[derive(Debug)]
pub(in crate::sol) struct DebugError {}
impl std::error::Error for DebugError {}
impl std::fmt::Display for DebugError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "debug error")
    }
}

pub(in crate::sol) type DebugResult = Result<(), DebugError>;

pub(in crate::sol) fn check_fit_key(uad: &Uad, fit_key: FitKey) -> DebugResult {
    if uad.fits.try_get(fit_key).is_none() {
        return Err(DebugError {});
    }
    Ok(())
}

pub(in crate::sol) fn check_item_key(uad: &Uad, item_key: ItemKey, check_load: bool) -> DebugResult {
    let item = match uad.items.try_get(item_key) {
        Some(item) => item,
        None => return Err(DebugError {}),
    };
    if check_load && !item.is_loaded() {
        return Err(DebugError {});
    }
    Ok(())
}

pub(in crate::sol) fn check_a_effect_id(uad: &Uad, a_effect_id: &ad::AEffectId) -> DebugResult {
    if uad.src.get_a_effect(a_effect_id).is_none() {
        return Err(DebugError {});
    }
    Ok(())
}

pub(in crate::sol) fn check_a_attr_id(uad: &Uad, a_attr_id: &ad::AAttrId) -> DebugResult {
    if uad.src.get_a_attr(a_attr_id).is_none() {
        return Err(DebugError {});
    }
    Ok(())
}
