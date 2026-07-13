use crate::{
    api::FleetMut,
    err::basic::{FitFoundError, FitNotInThisFleetError},
    ud::FitId,
};

impl<'s> FleetMut<'s> {
    pub fn add_fit(&mut self, fit_id: &FitId) -> Result<(), FleetAddFitError> {
        let fit_uid = self.sol.u_data.fits.int_id_by_ext_id_err(fit_id)?;
        let u_fit = self.sol.u_data.fits.get(fit_uid);
        if u_fit.fleet == Some(self.uid) {
            return Err(FitNotInThisFleetError {
                fit_id: *fit_id,
                fleet_id: self.sol.u_data.fleets.ext_id_by_int_id(self.uid),
            }
            .into());
        }
        self.sol.internal_set_fit_fleet(fit_uid, self.uid);
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
