use crate::{
    ss::SsView,
    util::{DebugError, DebugResult},
};

use super::TgtTracker;

impl TgtTracker {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        for (tgt_item_id, src_item_ids) in self.data.iter() {
            if ss_view.items.get_item(tgt_item_id).is_err() {
                return Err(DebugError::new());
            }
            for src_item_id in src_item_ids {
                if ss_view.items.get_item(src_item_id).is_err() {
                    return Err(DebugError::new());
                }
            }
        }
        Ok(())
    }
}
