use crate::{
    CmdResp, CmdResps, Fit, FitChangeEnumCmd, FitInfo, FitInfoCmdBr,
    err::{BrResolveError, FitChangeEnumError},
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, ctl_cmd: FitChangeEnumCmd) -> Result<CmdResp, FitChangeEnumError> {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                ctl_cmd.execute(&mut core_fit)
            })
            .await
    }
    #[tracing::instrument(name = "fit-chg-inf", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        ctl_cmd: FitChangeEnumCmd,
        info_cmd: FitInfoCmdBr,
    ) -> Result<(CmdResp, FitInfo), FitChangeEnumFitInfoError> {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                let ctl_cmd_resp = ctl_cmd.execute(&mut core_fit)?;
                let cmd_resps = CmdResps::with_resp(ctl_cmd_resp);
                let info_cmd = info_cmd.render(&cmd_resps)?;
                let fit_info = info_cmd.execute(&mut core_fit);
                let ctl_cmd_resp = cmd_resps.into_iter().next().unwrap();
                Ok((ctl_cmd_resp, fit_info))
            })
            .await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FitChangeEnumFitInfoError {
    #[error(transparent)]
    Change(#[from] FitChangeEnumError),
    #[error("failed to resolve backref in info command")]
    InfoBrResolve(#[from] BrResolveError),
}
