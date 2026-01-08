use crate::{
    api::{Fit, FitMut},
    err::basic::FitFoundError,
    sol::SolarSystem,
    ud::FitId,
};

impl SolarSystem {
    pub fn get_fit(&self, fit_id: &FitId) -> Result<Fit<'_>, GetFitError> {
        let fit_uid = self.u_data.fits.iid_by_xid_err(fit_id)?;
        Ok(Fit::new(self, fit_uid))
    }
    pub fn get_fit_mut(&mut self, fit_id: &FitId) -> Result<FitMut<'_>, GetFitError> {
        let fit_uid = self.u_data.fits.iid_by_xid_err(fit_id)?;
        Ok(FitMut::new(self, fit_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
