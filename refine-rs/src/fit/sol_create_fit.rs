use crate::{
    cmd::{CreateFitCmd, CreateFitError},
    fit::Fit,
    sol::SolarSystem,
};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "fit-crt", level = "trace", skip_all)]
    pub async fn create_fit(&'s mut self, cmd: CreateFitCmd) -> Result<Fit<'r, 's>, CreateFitError> {
        let cmd_resp = self
            .exec_standard_fallible(move |core_sol| cmd.execute(core_sol))
            .await?;
        Ok(Fit::new(self, cmd_resp.fit_id))
    }
}
