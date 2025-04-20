use crate::sol::{
    SolarSystem,
    api::{Fleet, MutIter, mut_iter::FleetMutGenerator},
};

impl SolarSystem {
    pub fn iter_fleets(&self) -> impl ExactSizeIterator<Item = Fleet> {
        self.uad.fleets.keys().map(|fleet_key| Fleet::new(self, fleet_key))
    }
    pub fn iter_fleets_mut(&mut self) -> MutIter<'_, FleetMutGenerator> {
        let fleet_keys = self.uad.fleets.keys().collect();
        MutIter::new(self, fleet_keys)
    }
}
