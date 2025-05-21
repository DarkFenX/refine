use crate::sol::{debug::DebugResult, svc::Svc, uad::Uad};

impl Svc {
    pub(in crate::sol) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        self.calc.consistency_check(uad)?;
        self.vast.consistency_check(uad)?;
        Ok(())
    }
}
