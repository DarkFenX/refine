use crate::{
    dbg::{DebugError, DebugResult},
    def::OF,
    ud::UProjData,
};

impl UProjData {
    pub(crate) fn consistency_check(&self) -> DebugResult {
        if self.range_s2s != (self.range_c2c - self.src_rad - self.tgt_rad).max(OF(0.0)) {
            return Err(DebugError {});
        }
        Ok(())
    }
}
