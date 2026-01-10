use crate::{
    dbg::{DebugError, DebugResult},
    num::PValue,
    ud::UProjData,
};

impl UProjData {
    pub(crate) fn consistency_check(&self) -> DebugResult {
        if self.range_s2s
            != PValue::from_f64_clamped(
                self.range_c2c.into_f64() - self.src_radius.into_f64() - self.tgt_radius.into_f64(),
            )
        {
            return Err(DebugError {});
        }
        Ok(())
    }
}
