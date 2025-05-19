use crate::sol::{SolarSystem, debug::DebugResult};

impl SolarSystem {
    // This function is intended to be used in tests, to make sure inner state is consistent, i.e.
    // no links broken, mutual references are correct, etc. All the entities which contain data
    // should be checked, and this function should be called from tests, to ensure there are no
    // memory leaks.
    pub fn internal_consistency_check(&self) -> DebugResult {
        // Check solar system structure
        self.uad.consistency_check()?;
        // Check solar system helper data containers
        self.proj_tracker.consistency_check(&self.uad)?;
        // Check services
        self.svc.consistency_check(&self.uad)?;
        Ok(())
    }
}
