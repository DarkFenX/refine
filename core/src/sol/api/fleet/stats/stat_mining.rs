use crate::{misc::Mining, sol::api::FleetMut, svc::vast::StatMiningItemKinds};

impl<'a> FleetMut<'a> {
    pub fn get_stat_mps_ore(&mut self, item_kinds: StatMiningItemKinds) -> Mining {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_mps_ore(&self.sol.u_data, u_fleet.iter_fits(), item_kinds)
    }
    pub fn get_stat_mps_ice(&mut self, item_kinds: StatMiningItemKinds) -> Mining {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_mps_ice(&self.sol.u_data, u_fleet.iter_fits(), item_kinds)
    }
    pub fn get_stat_mps_gas(&mut self, item_kinds: StatMiningItemKinds) -> Mining {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_mps_gas(&self.sol.u_data, u_fleet.iter_fits(), item_kinds)
    }
}
