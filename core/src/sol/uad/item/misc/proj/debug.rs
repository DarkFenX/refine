use crate::sol::{
    debug::{DebugResult, check_item_id},
    uad::Uad,
};

use super::Projs;

impl Projs {
    pub(in crate::sol::uad::item) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for projectee_item_id in self.iter_items() {
            check_item_id(uad, projectee_item_id, false)?;
        }
        Ok(())
    }
}
