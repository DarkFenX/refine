use crate::{
    dbg::{DebugError, DebugResult},
    ud::fit::item_vec::UItemVec,
};

impl UItemVec {
    pub(crate) fn consistency_check(&self) -> DebugResult {
        if self.data.iter().filter(|item_uid| item_uid.is_some()).count() != self.item_count {
            return Err(DebugError {});
        }
        Ok(())
    }
}
