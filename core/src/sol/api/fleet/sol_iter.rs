use crate::sol::{SolarSystem, api::Fleet};

impl SolarSystem {
    pub fn iter_fleets(&self) -> impl ExactSizeIterator<Item = Fleet> {
        self.uad.fleets.keys().map(|fleet_key| Fleet::new(self, fleet_key))
    }
}
