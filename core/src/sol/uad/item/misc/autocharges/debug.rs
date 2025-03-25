use crate::sol::{
    debug::{DebugResult, check_item_id},
    uad::Uad,
};

use super::Autocharges;

impl Autocharges {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for autocharge_id in self.values() {
            check_item_id(uad, autocharge_id, false)?;
        }
        Ok(())
    }
}
