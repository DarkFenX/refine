use crate::{
    def::FleetId,
    err::basic::FleetFoundError,
    sol::{SolarSystem, api::FitMut},
    ud::{UFitKey, UFleetKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_fleet(&mut self, fit_key: UFitKey, fleet_key: UFleetKey) {
        let u_fit = self.u_data.fits.get(fit_key);
        self.u_data.fleets.get(fleet_key);
        // Unassign from old fleet
        if let Some(old_fleet_key) = u_fit.fleet {
            if old_fleet_key == fleet_key {
                return;
            }
            let old_u_fleet = self.u_data.fleets.get(old_fleet_key);
            self.svc
                .notify_fit_removed_from_fleet(&self.u_data, old_u_fleet, &fit_key);
            let old_u_fleet = self.u_data.fleets.get_mut(old_fleet_key);
            old_u_fleet.remove_fit(&fit_key);
        }
        // Assign new fleet
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.fleet = Some(fleet_key);
        let u_fleet = self.u_data.fleets.get_mut(fleet_key);
        u_fleet.add_fit(fit_key);
        let u_fleet = self.u_data.fleets.get(fleet_key);
        self.svc.notify_fit_added_to_fleet(&self.u_data, u_fleet, &fit_key);
    }
}

impl<'a> FitMut<'a> {
    pub fn set_fleet(&mut self, fleet_id: &FleetId) -> Result<(), SetFitFleetError> {
        let fleet_key = self.sol.u_data.fleets.key_by_id_err(fleet_id)?;
        self.sol.internal_set_fit_fleet(self.key, fleet_key);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetFitFleetError {
    #[error("{0}")]
    FleetNotFound(#[from] FleetFoundError),
}
