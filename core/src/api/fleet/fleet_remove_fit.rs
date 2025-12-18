use crate::{
    api::FleetMut,
    def::FitId,
    err::basic::{FitFoundError, FitInThisFleetError},
};

impl<'a> FleetMut<'a> {
    pub fn remove_fit(&mut self, fit_id: &FitId) -> Result<(), FleetRemoveFitError> {
        let fit_key = self.sol.u_data.fits.key_by_id_err(fit_id)?;
        let u_fit = self.sol.u_data.fits.get(fit_key);
        if u_fit.fleet != Some(self.key) {
            return Err(FitInThisFleetError {
                fit_id: u_fit.id,
                fleet_id: self.sol.u_data.fleets.id_by_key(self.key),
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
