use crate::{
    api::{Fleet, FleetMut, MutIter},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn iter_fleets(&self) -> impl ExactSizeIterator<Item = Fleet<'_>> {
        self.u_data.fleets.keys().map(|fleet_uid| Fleet::new(self, fleet_uid))
    }
    pub fn iter_fleets_mut(&mut self) -> MutIter<'_, FleetMut<'_>> {
        let fleet_uids = self.u_data.fleets.keys().collect();
        MutIter::new(self, fleet_uids)
    }
}
