use crate::{Fit, FitId, SolarSystem};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "fit-get", level = "trace", skip_all)]
    pub async fn get_fit(&'s mut self, fit_id: FitId) -> Result<Fit<'r, 's>, FitGetError> {
        let fit_id = self.exec_inplace(|core_sol| core_sol.get_fit(&fit_id).map(|core_fit| core_fit.get_fit_id()))?;
        let fit = Fit::new(self, fit_id);
        Ok(fit)
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct FitGetError(#[from] pub rc::err::FitGetError);
