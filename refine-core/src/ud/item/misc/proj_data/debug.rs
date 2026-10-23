use crate::{dbg::DebugResult, num::PValue, ud::UProjData};

impl UProjData {
    pub(crate) fn consistency_check(&self) -> DebugResult {
        let Self {
            range_c2c,
            range_s2s,
            src_radius,
            tgt_radius,
            // Not checked
            src_physics: _,
            tgt_physics: _,
        } = self;
        if *range_s2s != PValue::from_f64_clamped(range_c2c.into_f64() - src_radius.into_f64() - tgt_radius.into_f64())
        {
            return Err(Default::default());
        }
        Ok(())
    }
}
