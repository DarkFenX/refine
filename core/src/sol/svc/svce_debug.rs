use crate::sol::{svc::SolSvc, uad::SolUad, SolDebugResult};

impl SolSvc {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &SolUad) -> SolDebugResult {
        self.running_effects.debug_consistency_check(uad)?;
        self.calc.debug_consistency_check(uad)?;
        self.rest.debug_consistency_check(uad)?;
        Ok(())
    }
}
