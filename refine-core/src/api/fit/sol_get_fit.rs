use crate::{Fit, FitId, Sol, err::basic::FitFoundError};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_fit(&mut self, fit_id: &FitId) -> Result<Fit<'_>, FitGetError> {
        let core = self.inner_mut();
        let fit_uid = core.u_data.fits.int_id_by_ext_id_err(fit_id)?;
        Ok(Fit::new(core, fit_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FitGetError {
    #[error(transparent)]
    FitNotFound(#[from] FitFoundError),
}
