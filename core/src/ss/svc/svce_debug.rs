use crate::{
    ss::{svc::SsSvcs, SsView},
    util::DebugResult,
};

impl SsSvcs {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        self.running_effects.debug_consistency_check(ss_view)?;
        self.calc_data.debug_consistency_check(ss_view)?;
        Ok(())
    }
}
