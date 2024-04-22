use crate::{
    sol::{svc::SolSvcs, SolView},
    util::DebugResult,
};

impl SolSvcs {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> DebugResult {
        self.running_effects.debug_consistency_check(sol_view)?;
        self.calc_data.debug_consistency_check(sol_view)?;
        Ok(())
    }
}
