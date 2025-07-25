use crate::{
    dbg::{DebugError, DebugResult},
    ud::UProjRange,
};

impl UProjRange {
    pub(crate) fn consistency_check(&self) -> DebugResult {
        if self.s2s + self.src_rad + self.tgt_rad != self.c2c {
            return Err(DebugError {});
        }
        Ok(())
    }
}
