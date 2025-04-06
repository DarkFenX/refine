use crate::sol::{
    debug::{DebugError, DebugResult},
    uad::Uad,
};

use super::ProjTracker;

impl ProjTracker {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        for (projectee_item_id, projector_item_ids) in self.data.iter() {
            if uad.items.get_by_id(projectee_item_id).is_err() {
                return Err(DebugError {});
            }
            for projector_item_id in projector_item_ids {
                if uad.items.get_by_id(projector_item_id).is_err() {
                    return Err(DebugError {});
                }
            }
        }
        Ok(())
    }
}
