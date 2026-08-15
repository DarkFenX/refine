use crate::{
    AddFitCmd, Fit, FitInfo, FitInfoArgs, SolarSystem,
    err::AddFitError,
    info::{FitInfoModesInt, ItemInfoModesInt},
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
    #[tracing::instrument(name = "fit-add-inf", level = "trace", skip_all)]
    pub async fn add_fit_and_get_info(
        &'s mut self,
        exec_cmd: AddFitCmd,
        info_args: FitInfoArgs,
    ) -> Result<(Fit<'r, 's>, FitInfo), AddFitError> {
        let (fit_id, fit_info) = self
            .exec_standard_fallible(move |core_sol| {
                let fit_id = exec_cmd.execute(core_sol)?.fit_id;
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                let fit_info_modes = FitInfoModesInt::from_pub_mode(info_args.fit);
                let item_info_modes = ItemInfoModesInt::from_pub_modes_regular(info_args.item);
                let fit_info = FitInfo::from_core(&mut core_fit, &fit_info_modes, &item_info_modes);
                Ok::<_, AddFitError>((fit_id, fit_info))
            })
            .await?;
        let fit = Fit::new(self, fit_id);
        Ok((fit, fit_info))
    }
}
