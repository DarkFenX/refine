use crate::{
    cmd::{AddFitCmd, AddFitError},
    fit::Fit,
    sol::SolarSystem,
};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "fit-add", level = "trace", skip_all)]
    pub async fn add_fit(&'s mut self, cmd: AddFitCmd) -> Result<Fit<'r, 's>, AddFitError> {
        let cmd_resp = self
            .exec_standard_fallible(move |core_sol| cmd.execute(core_sol))
            .await?;
        Ok(Fit::new(self, cmd_resp.fit_id))
    }
}
