use crate::sol::{
    SolarSystem,
    api::{Fleet, FleetMut, MutIter},
};

impl SolarSystem {
    pub fn iter_fleets(&self) -> impl ExactSizeIterator<Item = Fleet<'_>> {
        self.u_data.fleets.keys().map(|fleet_key| Fleet::new(self, fleet_key))
    }
    pub fn iter_fleets_mut(&mut self) -> MutIter<'_, FleetMut<'_>> {
        let fleet_keys = self.u_data.fleets.keys().collect();
        MutIter::new(self, fleet_keys)
    }
}
