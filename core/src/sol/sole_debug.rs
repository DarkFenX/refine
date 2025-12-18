use crate::{dbg::DebugResult, sol::SolarSystem};

impl SolarSystem {
    pub(crate) fn internal_consistency_check(&self) -> DebugResult {
        // Check solar system structure
        self.u_data.consistency_check()?;
        // Check services
        self.svc.consistency_check(&self.u_data)?;
        // Check solar system helper data containers
        self.rev_projs.consistency_check(&self.u_data)?;
        Ok(())
    }
}
