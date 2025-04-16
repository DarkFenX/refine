use crate::{
    err::basic::FleetFoundError,
    sol::{FitKey, FleetId, FleetKey, SolarSystem, api::FitMut},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_set_fit_fleet(&mut self, fit_key: FitKey, fleet_key: FleetKey) {
        let uad_fit = self.uad.fits.get(fit_key);
        self.uad.fleets.get(fleet_key);
        // Unassign from old fleet
        if let Some(old_fleet_key) = uad_fit.fleet {
            if old_fleet_key == fleet_key {
                return;
            }
            let old_uad_fleet = self.uad.fleets.get(old_fleet_key);
            self.svc.remove_fit_from_fleet(&self.uad, old_uad_fleet, &fit_key);
            let old_uad_fleet = self.uad.fleets.get_mut(old_fleet_key);
            old_uad_fleet.remove_fit(&fit_key);
        }
        // Assign new fleet
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.fleet = Some(fleet_key);
        let uad_fleet = self.uad.fleets.get_mut(fleet_key);
        uad_fleet.add_fit(fit_key);
        let uad_fleet = self.uad.fleets.get(fleet_key);
        self.svc.add_fit_to_fleet(&self.uad, uad_fleet, &fit_key);
    }
}

impl<'a> FitMut<'a> {
    pub fn set_fleet(self, fleet_id: &FleetId) -> Result<Self, SetFitFleetError> {
        let fleet_key = self.sol.uad.fleets.key_by_id_err(fleet_id)?;
        self.sol.internal_set_fit_fleet(self.key, fleet_key);
        Ok(self)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetFitFleetError {
    #[error("{0}")]
    FleetNotFound(#[from] FleetFoundError),
}
