use crate::{dbg::DebugResult, svc::Svc, uad::Uad};

impl Svc {
    pub(crate) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        self.calc.consistency_check(uad)?;
        self.vast.consistency_check(uad)?;
        self.eprojs.consistency_check(uad)?;
        Ok(())
    }
}
