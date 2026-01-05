use crate::{
    dbg::{DebugError, DebugResult},
    misc::PValue,
    ud::UProjData,
};

impl UProjData {
    pub(crate) fn consistency_check(&self) -> DebugResult {
        if self.range_s2s
            != PValue::new_clamped(
                self.range_c2c.into_inner() - self.src_radius.into_inner() - self.tgt_radius.into_inner(),
            )
        {
            return Err(DebugError {});
        }
        Ok(())
    }
}
