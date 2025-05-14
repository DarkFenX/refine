use crate::{
    err::basic::{FitFoundError, FitNotInThisFleetError},
    sol::{FitId, api::FleetMut},
};

impl<'a> FleetMut<'a> {
    pub fn add_fit(&mut self, fit_id: &FitId) -> Result<(), FleetAddFitError> {
        let fit_key = self.sol.uad.fits.key_by_id_err(fit_id)?;
        let uad_fit = self.sol.uad.fits.get(fit_key);
        if uad_fit.fleet == Some(self.key) {
            return Err(FitNotInThisFleetError {
                fit_id: *fit_id,
                fleet_id: self.sol.uad.fleets.id_by_key(self.key),
            }
            .into());
        }
        self.sol.internal_set_fit_fleet(fit_key, self.key);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FleetAddFitError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
    #[error("{0}")]
    FitAlreadyInThisFleet(#[from] FitNotInThisFleetError),
}
