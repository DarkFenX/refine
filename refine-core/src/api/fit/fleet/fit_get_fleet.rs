use crate::api::{Fit, Fleet};

impl<'s> Fit<'s> {
    pub fn get_fleet(&mut self) -> Option<Fleet<'_>> {
        let u_fleet = self.sol.u_data.fits.get(self.uid);
        u_fleet.fleet.map(|fleet_uid| Fleet::new(self.sol, fleet_uid))
    }
}
