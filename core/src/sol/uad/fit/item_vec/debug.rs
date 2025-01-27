use crate::sol::debug::{SolDebugError, SolDebugResult};

use super::SolItemVec;

impl SolItemVec {
    pub(in crate::sol) fn debug_consistency_check(&self) -> SolDebugResult {
        if self.data.iter().filter(|v| v.is_some()).count() != self.item_count {
            return Err(SolDebugError::new());
        }
        Ok(())
    }
}
