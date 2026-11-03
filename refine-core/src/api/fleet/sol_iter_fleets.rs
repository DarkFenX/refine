use crate::{
    api::{EntityIter, Fleet},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn iter_fleets(&mut self) -> EntityIter<'_, Fleet<'_>> {
        let fleet_uids = self.u_data.fleets.keys().collect();
        EntityIter::new(self, fleet_uids)
    }
}
