use crate::{AddFitCmd, Fit, FitInfo, FitInfoModes, SolarSystem, err::AddFitError};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "fit-add", level = "trace", skip_all)]
    pub async fn add_fit(&'s mut self, cmd: AddFitCmd) -> Result<Fit<'r, 's>, AddFitError> {
        let cmd_resp = self
            .exec_standard_fallible(move |core_sol| cmd.execute(core_sol))
            .await?;
        let fit = Fit::new(self, cmd_resp.fit_id);
        Ok(fit)
    }
    #[tracing::instrument(name = "fit-add-inf", level = "trace", skip_all)]
    pub async fn add_fit_and_get_info(
        &'s mut self,
        exec_cmd: AddFitCmd,
        info_modes: FitInfoModes,
    ) -> Result<(Fit<'r, 's>, FitInfo), AddFitError> {
        let (fit_id, fit_info) = self
            .exec_standard_fallible(move |core_sol| {
                let fit_id = exec_cmd.execute(core_sol)?.fit_id;
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                let fit_info = FitInfo::from_core(&mut core_fit, info_modes);
                Ok::<_, AddFitError>((fit_id, fit_info))
            })
            .await?;
        let fit = Fit::new(self, fit_id);
        Ok((fit, fit_info))
    }
}
