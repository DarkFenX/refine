use crate::sol::{
    debug::{SolDebugResult, check_item},
    uad::SolUad,
};

use super::SolProjs;

impl SolProjs {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for projectee_item_id in self.iter_items() {
            check_item(uad, projectee_item_id, false)?;
        }
        Ok(())
    }
}
