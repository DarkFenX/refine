use itertools::Itertools;

use crate::{
    err::basic::FleetFoundError,
    sol::{FleetId, FleetKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_fleet(&mut self, fleet_id: &FleetId) -> Result<(), RemoveFleetError> {
        let fleet_key = self.uad.fleets.key_by_id_err(fleet_id)?;
        self.remove_fleet_internal(fleet_key);
        Ok(())
    }
    pub(in crate::sol) fn remove_fleet_internal(&mut self, fleet_key: FleetKey) {
        let fleet = self.uad.fleets.get(fleet_key);
        let fit_keys = fleet.iter_fits().copied().collect_vec();
        for fit_key in fit_keys {
            self.svc.remove_fit_from_fleet(&self.uad, fleet, &fit_key);
            let fit = self.uad.fits.get_mut(fit_key);
            fit.fleet = None;
        }
        self.uad.fleets.remove(fleet_key);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveFleetError {
    #[error("{0}")]
    FleetNotFound(#[from] FleetFoundError),
}
