use crate::{
    api::{Fit, FitMut},
    err::basic::FitFoundError,
    sol::SolarSystem,
    ud::FitId,
};

impl SolarSystem {
    pub fn get_fit(&self, fit_id: &FitId) -> Result<Fit<'_>, FitGetError> {
        let fit_uid = self.u_data.fits.int_id_by_ext_id_err(fit_id)?;
        Ok(Fit::new(self, fit_uid))
    }
    pub fn get_fit_mut(&mut self, fit_id: &FitId) -> Result<FitMut<'_>, FitGetError> {
        let fit_uid = self.u_data.fits.int_id_by_ext_id_err(fit_id)?;
        Ok(FitMut::new(self, fit_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FitGetError {
    #[error(transparent)]
    FitNotFound(#[from] FitFoundError),
}
