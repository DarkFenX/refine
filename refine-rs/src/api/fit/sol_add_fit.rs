use crate::{Fit, FitAddCmd, FitInfo, FitInfoCmd, SolarSystem, err::FitAddError, shared::SolBackup};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "fit-add", level = "trace", skip_all)]
    pub async fn add_fit(&'s mut self, ctl_cmd: FitAddCmd) -> Result<Fit<'r, 's>, FitAddError> {
        let ctl_cmd_resp = self
            .exec_standard(SolBackup::Needed, move |core_sol| ctl_cmd.execute(core_sol))
            .await?;
        let fit = Fit::new(self, ctl_cmd_resp.fit_id);
        Ok(fit)
    }
    #[tracing::instrument(name = "fit-add-inf", level = "trace", skip_all)]
    pub async fn add_fit_and_get_info(
        &'s mut self,
        ctl_cmd: FitAddCmd,
        info_cmd: FitInfoCmd,
    ) -> Result<(Fit<'r, 's>, FitInfo), FitAddError> {
        let (fit_id, fit_info) = self
            .exec_standard(SolBackup::Needed, move |core_sol| {
                let fit_id = ctl_cmd.execute(core_sol)?.fit_id;
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                let fit_info = info_cmd.execute(&mut core_fit);
                Ok::<_, FitAddError>((fit_id, fit_info))
            })
            .await?;
        let fit = Fit::new(self, fit_id);
        Ok((fit, fit_info))
    }
}
