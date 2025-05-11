use super::ProjTracker;
use crate::sol::{
    debug::{DebugError, DebugResult},
    uad::Uad,
};

impl ProjTracker {
    pub(in crate::sol) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        for (projectee_item_key, projector_item_keys) in self.data.iter() {
            if uad.items.try_get(*projectee_item_key).is_none() {
                return Err(DebugError {});
            }
            for projector_item_key in projector_item_keys {
                if uad.items.try_get(*projector_item_key).is_none() {
                    return Err(DebugError {});
                }
            }
        }
        Ok(())
    }
}
