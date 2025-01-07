use crate::sol::{
    uad::{item::debug, SolUad},
    SolDebugResult,
};

use super::SolAutocharges;

impl SolAutocharges {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for autocharge_id in self.values() {
            debug::check_item(uad, autocharge_id)?;
        }
        Ok(())
    }
}
