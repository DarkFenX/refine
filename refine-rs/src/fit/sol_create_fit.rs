use crate::{
    cmd::{CreateFitCmd, CreateFitError},
    fit::Fit,
    sol::SolarSystem,
};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "fit-crt", level = "trace", skip_all)]
    pub async fn create_fit(&'s mut self, cmd: CreateFitCmd) -> Result<Fit<'r, 's>, CreateFitError> {
        self.exec_standard_fallible(move |core_sol| cmd.execute(core_sol))
            .await
            .map(|cmd_resp| Fit::new(self, cmd_resp.fit_id))
    }
}
