use crate::{
    sol::api::FleetMut,
    svc::vast::{StatMiningItemKinds, StatMiningKinds},
};

impl<'a> FleetMut<'a> {
    pub fn get_stat_mps(&mut self, item_kinds: StatMiningItemKinds) -> StatMiningKinds {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_mps(&self.sol.u_data, u_fleet.iter_fits(), item_kinds)
    }
}
