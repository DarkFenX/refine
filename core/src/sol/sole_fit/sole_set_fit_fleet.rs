use crate::{
    err::basic::{FitFoundError, FleetFoundError},
    sol::{FitId, FitKey, FleetId, FleetKey, SolarSystem},
};

impl SolarSystem {
    pub fn set_fit_fleet(&mut self, fit_id: &FitId, fleet_id: &FleetId) -> Result<(), SetFitFleetError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let fleet_key = self.uad.fleets.key_by_id_err(fleet_id)?;
        self.set_fit_fleet_internal(fit_key, fleet_key);
        Ok(())
    }
    pub(in crate::sol) fn set_fit_fleet_internal(&mut self, fit_key: FitKey, fleet_key: FleetKey) {
        let fit = self.uad.fits.get(fit_key);
        self.uad.fleets.get(fleet_key);
        // Unassign from old fleet
        if let Some(old_fleet_key) = fit.fleet {
            if old_fleet_key == fleet_key {
                return;
            }
            let old_fleet = self.uad.fleets.get(old_fleet_key);
            self.svc.remove_fit_from_fleet(&self.uad, old_fleet, &fit_key);
            let old_fleet = self.uad.fleets.get_mut(old_fleet_key);
            old_fleet.remove_fit(&fit_key);
        }
        // Assign new fleet
        let fit = self.uad.fits.get_mut(fit_key);
        fit.fleet = Some(fleet_key);
        let fleet = self.uad.fleets.get_mut(fleet_key);
        fleet.add_fit(fit_key);
        let fleet = self.uad.fleets.get(fleet_key);
        self.svc.add_fit_to_fleet(&self.uad, fleet, &fit_key);
    }
}

#[derive(Debug)]
pub enum SetFitFleetError {
    FitNotFound(FitFoundError),
    FleetNotFound(FleetFoundError),
}
impl std::error::Error for SetFitFleetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::FleetNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitFleetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::FleetNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitFleetError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<FleetFoundError> for SetFitFleetError {
    fn from(error: FleetFoundError) -> Self {
        Self::FleetNotFound(error)
    }
}
