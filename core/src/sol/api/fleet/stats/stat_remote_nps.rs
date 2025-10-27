use crate::{def::AttrVal, sol::api::FleetMut};

impl<'a> FleetMut<'a> {
    pub fn get_stat_remote_nps(&mut self) -> AttrVal {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_remote_nps(&self.sol.u_data, u_fleet.iter_fits())
    }
}
