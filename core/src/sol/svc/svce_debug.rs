use crate::sol::{svc::SolSvcs, SolDebugResult, SolView};

impl SolSvcs {
    pub(in crate::sol) fn debug_consistency_check(&self, sol_view: &SolView) -> SolDebugResult {
        self.running_effects.debug_consistency_check(sol_view)?;
        self.calc.debug_consistency_check(sol_view)?;
        self.stats.debug_consistency_check(sol_view)?;
        Ok(())
    }
}
