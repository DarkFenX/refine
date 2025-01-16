use crate::sol::{
    debug::{SolDebugError, SolDebugResult},
    uad::SolUad,
};

use super::SolProjTracker;

impl SolProjTracker {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        for (projectee_item_id, projector_item_ids) in self.data.iter() {
            if uad.items.get_item(projectee_item_id).is_err() {
                return Err(SolDebugError::new());
            }
            for projector_item_id in projector_item_ids {
                if uad.items.get_item(projector_item_id).is_err() {
                    return Err(SolDebugError::new());
                }
            }
        }
        Ok(())
    }
}
