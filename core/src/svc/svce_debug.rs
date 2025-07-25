use crate::{dbg::DebugResult, svc::Svc, ud::UData};

impl Svc {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.calc.consistency_check(u_data)?;
        self.vast.consistency_check(u_data)?;
        self.eprojs.consistency_check(u_data)?;
        Ok(())
    }
}
