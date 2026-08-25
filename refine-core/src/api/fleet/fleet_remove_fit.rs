use crate::{FleetId, FleetMut, err::basic::FitFoundError, ud::FitId};

impl<'s> FleetMut<'s> {
    pub fn remove_fit(&mut self, fit_id: &FitId) -> Result<(), FleetFitRemoveError> {
        let fit_uid = self.sol.u_data.fits.int_id_by_ext_id_err(fit_id)?;
        let u_fit = self.sol.u_data.fits.get(fit_uid);
        if u_fit.fleet != Some(self.uid) {
            return Err(FleetFitRemoveError::FitIsNotInThisFleet(
                self.sol.u_data.fleets.ext_id_by_int_id(self.uid),
                u_fit.id,
            ));
        }
        self.sol.internal_unset_fit_fleet(fit_uid).unwrap();
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FleetFitRemoveError {
    #[error(transparent)]
    FitNotFound(#[from] FitFoundError),
    #[error("fit {1} is not a member of fleet {0}")]
    FitIsNotInThisFleet(FleetId, FitId),
}
