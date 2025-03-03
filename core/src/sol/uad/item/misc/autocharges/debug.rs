use crate::sol::{
    debug::{SolDebugResult, check_item},
    uad::SolUad,
};

use super::SolAutocharges;

impl SolAutocharges {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for autocharge_id in self.values() {
            check_item(uad, autocharge_id, false)?;
        }
        Ok(())
    }
}
