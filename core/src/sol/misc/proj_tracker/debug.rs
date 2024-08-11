use crate::sol::{SolDebugError, SolDebugResult, SolView};

use super::SolProjTracker;

impl SolProjTracker {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        for (projectee_item_id, projector_item_ids) in self.data.iter() {
            if sol_view.items.get_item(projectee_item_id).is_err() {
                return Err(SolDebugError::new());
            }
            for projector_item_id in projector_item_ids {
                if sol_view.items.get_item(projector_item_id).is_err() {
                    return Err(SolDebugError::new());
                }
            }
        }
        Ok(())
    }
}
