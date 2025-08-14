use crate::{
    dbg::{DebugError, DebugResult},
    ud::UProjData,
};

impl UProjData {
    pub(crate) fn consistency_check(&self) -> DebugResult {
        if self.range_s2s + self.src_rad + self.tgt_rad != self.range_c2c {
            return Err(DebugError {});
        }
        Ok(())
    }
}
