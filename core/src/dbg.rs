use crate::{
    rd::{RAttrId, REffectId},
    ud::{UData, UFitId, UItemId},
};

#[derive(thiserror::Error, Debug)]
#[error("debug error")]
pub(crate) struct DebugError {}

pub(crate) type DebugResult = Result<(), DebugError>;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Definitions for a few basic entities
////////////////////////////////////////////////////////////////////////////////////////////////////
impl UFitId {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if u_data.fits.try_get(*self).is_none() {
            return Err(DebugError {});
        }
        Ok(())
    }
}

impl UItemId {
    pub(crate) fn consistency_check(&self, u_data: &UData, check_load: bool) -> DebugResult {
        let item = match u_data.items.try_get(*self) {
            Some(item) => item,
            None => return Err(DebugError {}),
        };
        if check_load && !item.is_loaded() {
            return Err(DebugError {});
        }
        Ok(())
    }
}

impl RAttrId {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        // Will crash if attr ID is not valid
        u_data.src.get_attr_by_rid(*self);
        Ok(())
    }
}

impl REffectId {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        // Will crash if effect ID is not valid
        u_data.src.get_effect_by_rid(*self);
        Ok(())
    }
}
