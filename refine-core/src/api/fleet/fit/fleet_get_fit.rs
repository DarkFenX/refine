use crate::{
    Fit, FitId, Fleet, FleetId,
    err::basic::FitFoundError,
    sol::SolCore,
    ud::{UFitId, UFleetId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fleet<'s> {
    pub fn get_fit(&mut self, fit_id: &FitId) -> Result<Fit<'_>, FleetFitGetError> {
        let fit_uid = self.sol.internal_get_fleet_fit_uid(self.uid, fit_id)?;
        Ok(Fit::new(self.sol, fit_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FleetFitGetError {
    #[error(transparent)]
    FitNotFound(#[from] FitFoundError),
    #[error("fit {1} is not a member of fleet {0}")]
    FitIsNotInThisFleet(FleetId, FitId),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_get_fleet_fit_uid(
        &self,
        fleet_uid: UFleetId,
        fit_id: &FitId,
    ) -> Result<UFitId, FleetFitGetError> {
        let fit_uid = self.u_data.fits.int_id_by_ext_id_err(fit_id)?;
        if self.u_data.fits.get(fit_uid).fleet != Some(fleet_uid) {
            return Err(FleetFitGetError::FitIsNotInThisFleet(
                self.u_data.fleets.ext_id_by_int_id(fleet_uid),
                *fit_id,
            ));
        }
        Ok(fit_uid)
    }
}
