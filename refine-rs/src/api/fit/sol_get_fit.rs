use crate::{Fit, FitId, Sol};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'refine, 'sol> Sol<'refine> {
    #[tracing::instrument(name = "fit-get", level = "error", skip_all)]
    pub async fn get_fit(&'sol mut self, fit_id: FitId) -> Result<Fit<'refine, 'sol>, FitGetError> {
        let fit_id = self.exec_inplace(|core_sol| core_sol.get_fit(&fit_id).map(|core_fit| core_fit.get_fit_id()))?;
        let fit = Fit::new(self, fit_id);
        Ok(fit)
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct FitGetError(#[from] pub crate::err::core::FitGetError);
