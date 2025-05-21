use crate::sol::{SolarSystem, debug::DebugResult};

impl SolarSystem {
    pub(in crate::sol) fn internal_consistency_check(&self) -> DebugResult {
        // Check solar system structure
        self.uad.consistency_check()?;
        // Check services
        self.svc.consistency_check(&self.uad)?;
        // Check solar system helper data containers
        self.reffs.consistency_check(&self.uad)?;
        self.rprojs.consistency_check(&self.uad)?;
        Ok(())
    }
}
