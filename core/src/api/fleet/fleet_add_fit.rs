use crate::{
    api::FleetMut,
    def::FitId,
    err::basic::{FitFoundError, FitNotInThisFleetError},
};

impl<'a> FleetMut<'a> {
    pub fn add_fit(&mut self, fit_id: &FitId) -> Result<(), FleetAddFitError> {
        let fit_key = self.sol.u_data.fits.iid_by_eid_err(fit_id)?;
        let u_fit = self.sol.u_data.fits.get(fit_key);
        if u_fit.fleet == Some(self.key) {
            return Err(FitNotInThisFleetError {
                fit_id: *fit_id,
                fleet_id: self.sol.u_data.fleets.eid_by_iid(self.key),
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
