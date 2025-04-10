use crate::sol::{
    debug::{DebugResult, check_item_key},
    uad::Uad,
};

use super::Autocharges;

impl Autocharges {
    pub(in crate::sol::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for autocharge_key in self.values() {
            // All autocharges are supposed to be loaded
            check_item_key(uad, *autocharge_key, true)?;
        }
        Ok(())
    }
}
