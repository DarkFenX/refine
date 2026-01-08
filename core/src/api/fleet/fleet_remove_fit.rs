use crate::{
    api::FleetMut,
    err::basic::{FitFoundError, FitInThisFleetError},
    ud::FitId,
};

impl<'a> FleetMut<'a> {
    pub fn remove_fit(&mut self, fit_id: &FitId) -> Result<(), FleetRemoveFitError> {
        let fit_uid = self.sol.u_data.fits.iid_by_xid_err(fit_id)?;
        let u_fit = self.sol.u_data.fits.get(fit_uid);
        if u_fit.fleet != Some(self.uid) {
            return Err(FitInThisFleetError {
                fit_id: u_fit.id,
                fleet_id: self.sol.u_data.fleets.xid_by_iid(self.uid),
            }
            .into());
        }
        self.sol.internal_unset_fit_fleet(fit_uid).unwrap();
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
