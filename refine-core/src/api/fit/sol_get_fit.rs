use crate::{Fit, FitId, Sol, err::basic::FitFoundError, sol::SolCore, ud::UFitId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_fit(&mut self, fit_id: &FitId) -> Result<Fit<'_>, FitGetError> {
        let sol = self.inner_mut();
        let fit_uid = sol.internal_get_sol_fit_uid(fit_id)?;
        Ok(Fit::new(sol, fit_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FitGetError {
    #[error(transparent)]
    FitNotFound(#[from] FitFoundError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_get_sol_fit_uid(&self, fit_id: &FitId) -> Result<UFitId, FitGetError> {
        Ok(self.u_data.fits.int_id_by_ext_id_err(fit_id)?)
    }
}
