use crate::{
    cmd::{AddFitCmd, AddFitError},
    fit::Fit,
    info::{FitInfo, FitInfoMode, ItemInfoMode},
    sol::SolarSystem,
};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "fit-add", level = "trace", skip_all)]
    pub async fn add_fit(&'s mut self, cmd: AddFitCmd) -> Result<Fit<'r, 's>, AddFitError> {
        let cmd_resp = self
            .exec_standard_fallible(move |core_sol| cmd.execute(core_sol))
            .await?;
        let fit = Fit::new(self, cmd_resp.fit_id);
        Ok(fit)
    }
    #[tracing::instrument(name = "fit-add", level = "trace", skip_all)]
    pub async fn add_fit_and_get_info(
        &'s mut self,
        cmd: AddFitCmd,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Result<(Fit<'r, 's>, FitInfo), AddFitError> {
        let (fit_id, fit_info) = self
            .exec_standard_fallible(move |core_sol| {
                let fit_id = cmd.execute(core_sol)?.fit_id;
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                let fit_info = FitInfo::from_core(&mut core_fit, fit_mode, item_mode);
                Ok::<_, AddFitError>((fit_id, fit_info))
            })
            .await?;
        let fit = Fit::new(self, fit_id);
        Ok((fit, fit_info))
    }
}
