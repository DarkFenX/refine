use crate::{fit::Fit, sol::SolarSystem};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "fit-get", level = "trace", skip_all)]
    pub async fn get_fit(&'s mut self, fit_id: rc::FitId) -> Result<Fit<'r, 's>, GetFitError> {
        let fit_id =
            self.exec_inplace(move |core_sol| core_sol.get_fit(&fit_id).map(|core_fit| core_fit.get_fit_id()))?;
        Ok(Fit::new(self, fit_id))
    }
}

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct GetFitError(#[from] pub rc::err::GetFitError);
