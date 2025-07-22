use crate::{
    def::FleetId,
    err::basic::FleetFoundError,
    sol::{SolarSystem, api::FitMut},
    uad::{UadFitKey, UadFleetKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_fleet(&mut self, fit_key: UadFitKey, fleet_key: UadFleetKey) {
        let uad_fit = self.uad.fits.get(fit_key);
        self.uad.fleets.get(fleet_key);
        // Unassign from old fleet
        if let Some(old_fleet_key) = uad_fit.fleet {
            if old_fleet_key == fleet_key {
                return;
            }
            let old_uad_fleet = self.uad.fleets.get(old_fleet_key);
            self.svc
                .notify_fit_removed_from_fleet(&self.uad, old_uad_fleet, &fit_key);
            let old_uad_fleet = self.uad.fleets.get_mut(old_fleet_key);
            old_uad_fleet.remove_fit(&fit_key);
        }
        // Assign new fleet
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.fleet = Some(fleet_key);
        let uad_fleet = self.uad.fleets.get_mut(fleet_key);
        uad_fleet.add_fit(fit_key);
        let uad_fleet = self.uad.fleets.get(fleet_key);
        self.svc.notify_fit_added_to_fleet(&self.uad, uad_fleet, &fit_key);
    }
}

impl<'a> FitMut<'a> {
    pub fn set_fleet(&mut self, fleet_id: &FleetId) -> Result<(), SetFitFleetError> {
        let fleet_key = self.sol.uad.fleets.key_by_id_err(fleet_id)?;
        self.sol.internal_set_fit_fleet(self.key, fleet_key);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SetFitFleetError {
    #[error("{0}")]
    FleetNotFound(#[from] FleetFoundError),
}
