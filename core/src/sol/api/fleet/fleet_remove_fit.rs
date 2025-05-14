use crate::{
    err::basic::{FitFoundError, FitInThisFleetError},
    sol::{FitId, api::FleetMut},
};

impl<'a> FleetMut<'a> {
    pub fn remove_fit(&mut self, fit_id: &FitId) -> Result<(), FleetRemoveFitError> {
        let fit_key = self.sol.uad.fits.key_by_id_err(fit_id)?;
        let uad_fit = self.sol.uad.fits.get(fit_key);
        if uad_fit.fleet != Some(self.key) {
            return Err(FitInThisFleetError {
                fit_id: uad_fit.id,
                fleet_id: self.sol.uad.fleets.id_by_key(self.key),
            }
            .into());
        }
        self.sol.internal_unset_fit_fleet(fit_key).unwrap();
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FleetRemoveFitError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
    #[error("{0}")]
    FitIsNotInThisFleet(#[from] FitInThisFleetError),
}
