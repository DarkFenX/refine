use crate::{Fit, FitId, Fleet, FleetId, err::basic::FitFoundError};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fleet<'s> {
    pub fn get_fit(&mut self, fit_id: &FitId) -> Result<Fit<'_>, FleetFitGetError> {
        let fit_uid = self.core.u_data.fits.int_id_by_ext_id_err(fit_id)?;
        if self.core.u_data.fits.get(fit_uid).fleet != Some(self.uid) {
            return Err(FleetFitGetError::FitIsNotInThisFleet(
                self.core.u_data.fleets.ext_id_by_int_id(self.uid),
                *fit_id,
            ));
        }
        Ok(Fit::new(self.core, fit_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FleetFitGetError {
    #[error(transparent)]
    FitNotFound(#[from] FitFoundError),
    #[error("fit {1} is not a member of fleet {0}")]
    FitIsNotInThisFleet(FleetId, FitId),
}
