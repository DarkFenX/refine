use crate::sol::{debug::DebugResult, svc::Svc, uad::Uad};

impl Svc {
    pub(in crate::sol) fn debug_consistency_check(&self, uad: &Uad) -> DebugResult {
        self.running_effects.debug_consistency_check(uad)?;
        self.calc.debug_consistency_check(uad)?;
        self.vast.debug_consistency_check(uad)?;
        Ok(())
    }
}
