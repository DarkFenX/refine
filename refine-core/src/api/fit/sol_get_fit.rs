use crate::{Fit, FitId, Sol, err::basic::FitFoundError};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_fit(&mut self, fit_id: &FitId) -> Result<Fit<'_>, FitGetError> {
        let sol = self.inner_mut();
        let fit_uid = sol.u_data.fits.int_id_by_ext_id_err(fit_id)?;
        Ok(Fit::new(sol, fit_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FitGetError {
    #[error(transparent)]
    FitNotFound(#[from] FitFoundError),
}
