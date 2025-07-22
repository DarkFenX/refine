use crate::{
    dbg::{DebugError, DebugResult},
    uad::fit::item_vec::UadItemVec,
};

impl UadItemVec {
    pub(crate) fn consistency_check(&self) -> DebugResult {
        if self.data.iter().filter(|v| v.is_some()).count() != self.item_count {
            return Err(DebugError {});
        }
        Ok(())
    }
}
