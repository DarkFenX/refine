use crate::{dbg::DebugResult, svc::Svc, ud::UData};

impl Svc {
    pub(crate) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        let Self { calc, vast, eff_projs } = self;
        calc.consistency_check(u_data)?;
        vast.consistency_check(u_data)?;
        eff_projs.consistency_check(u_data)?;
        Ok(())
    }
}
