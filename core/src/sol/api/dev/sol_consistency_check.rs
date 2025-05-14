use crate::sol::SolarSystem;

impl SolarSystem {
    // This function is intended to be used in tests, to make sure inner state is consistent, i.e.
    // no links broken, mutual references are correct, etc. All the entities which contain data
    // should be checked, and this function should be called from tests, to ensure there are no
    // memory leaks.
    pub fn consistency_check(&self) -> bool {
        // Check solar system structure
        if self.uad.consistency_check().is_err() {
            return false;
        }
        // Check solar system helper data containers
        if self.proj_tracker.consistency_check(&self.uad).is_err() {
            return false;
        }
        // Check services
        if self.svc.consistency_check(&self.uad).is_err() {
            return false;
        }
        true
    }
}
