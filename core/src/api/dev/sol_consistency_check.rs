use crate::sol::SolarSystem;

impl SolarSystem {
    // This function is intended to be used in tests, to make sure inner state is consistent, i.e.
    // no links broken, mutual references are correct, etc. All the entities which contain data
    // should be checked, and this function should be called from tests, to ensure there are no
    // memory leaks.
    pub fn consistency_check(&self) -> bool {
        self.internal_consistency_check().is_ok()
    }
}
