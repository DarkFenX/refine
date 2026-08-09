use crate::{FleetId, FleetMut, err::basic::FitFoundError, ud::FitId};

impl<'s> FleetMut<'s> {
    pub fn add_fit(&mut self, fit_id: &FitId) -> Result<(), FleetAddFitError> {
        let fit_uid = self.sol.u_data.fits.int_id_by_ext_id_err(fit_id)?;
        let u_fit = self.sol.u_data.fits.get(fit_uid);
        if u_fit.fleet == Some(self.uid) {
            return Err(FleetAddFitError::FitAlreadyInThisFleet(
                self.sol.u_data.fleets.ext_id_by_int_id(self.uid),
                *fit_id,
            ));
        }
        self.sol.internal_set_fit_fleet(fit_uid, self.uid);
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FleetAddFitError {
    #[error(transparent)]
    FitNotFound(#[from] FitFoundError),
    #[error("fit {1} is already a member of fleet {0}")]
    FitAlreadyInThisFleet(FleetId, FitId),
}
