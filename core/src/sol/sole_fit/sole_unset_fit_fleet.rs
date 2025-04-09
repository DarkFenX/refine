use crate::{
    err::basic::{FitFleetAssignedError, FitFoundError},
    sol::{FitId, FitKey, SolarSystem},
};

impl SolarSystem {
    pub fn unset_fit_fleet(&mut self, fit_id: &FitId) -> Result<(), UnsetFitFleetError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.unset_fit_fleet_internal(fit_key)?)
    }
    pub(in crate::sol) fn unset_fit_fleet_internal(&mut self, fit_key: FitKey) -> Result<(), FitFleetAssignedError> {
        let fit = self.uad.fits.get(fit_key);
        let fleet_key = match fit.fleet {
            Some(fleet_key) => fleet_key,
            None => return Err(FitFleetAssignedError { fit_id: fit.id }),
        };
        let fleet = self.uad.fleets.get(fleet_key);
        self.svc.remove_fit_from_fleet(&self.uad, fleet, &fit_key);
        let fleet = self.uad.fleets.get_mut(fleet_key);
        fleet.remove_fit(&fit_key);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.fleet = None;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum UnsetFitFleetError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
    #[error("{0}")]
    FitHasNoFleet(#[from] FitFleetAssignedError),
}
