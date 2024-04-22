use crate::{
    sol::SolView,
    util::{DebugError, DebugResult},
};

use super::SolTgtTracker;

impl SolTgtTracker {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        for (tgt_item_id, src_item_ids) in self.data.iter() {
            if sol_view.items.get_item(tgt_item_id).is_err() {
                return Err(DebugError::new());
            }
            for src_item_id in src_item_ids {
                if sol_view.items.get_item(src_item_id).is_err() {
                    return Err(DebugError::new());
                }
            }
        }
        Ok(())
    }
}
