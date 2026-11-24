use crate::{
    Fit, Sol,
    ctl::{FitAddCmd, err::FitAddError},
    info::{FitInfo, FitInfoCmd},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'refine, 'sol> Sol<'refine> {
    #[tracing::instrument(name = "fit-add", level = "error", skip_all)]
    pub async fn add_fit(&'sol mut self, ctl_cmd: FitAddCmd) -> Result<Fit<'refine, 'sol>, FitAddError> {
        let ctl_cmd_resp = self.exec_standard(|core_sol| ctl_cmd.execute(core_sol)).await?;
        let fit = Fit::new(self, ctl_cmd_resp.fit_id);
        Ok(fit)
    }
    #[tracing::instrument(name = "fit-add-inf", level = "error", skip_all)]
    pub async fn add_fit_and_get_info(
        &'sol mut self,
        ctl_cmd: FitAddCmd,
        info_cmd: FitInfoCmd,
    ) -> Result<(Fit<'refine, 'sol>, FitInfo), FitAddError> {
        let (fit_id, fit_info) = self
            .exec_standard(|core_sol| {
                let fit_id = ctl_cmd.execute(core_sol)?.fit_id;
                let mut core_fit = core_sol.get_fit(&fit_id).unwrap();
                let fit_info = info_cmd.execute(&mut core_fit);
                Ok::<_, FitAddError>((fit_id, fit_info))
            })
            .await?;
        let fit = Fit::new(self, fit_id);
        Ok((fit, fit_info))
    }
}
