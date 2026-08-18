use crate::{
    CmdResp, CmdResps, SolChangeEnumCmd, SolInfo, SolInfoCmdBr, SolarSystem,
    err::{BrResolveError, SolChangeEnumError},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, ctl_cmd: SolChangeEnumCmd) -> Result<CmdResp, SolChangeEnumError> {
        self.exec_standard_fallible(move |core_sol| ctl_cmd.execute(core_sol))
            .await
    }
    #[tracing::instrument(name = "sol-chg-inf", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        ctl_cmd: SolChangeEnumCmd,
        info_cmd: SolInfoCmdBr,
    ) -> Result<(CmdResp, SolInfo), SolChangeEnumSolInfoError> {
        self.exec_standard_fallible_ctx(|ctx, core_sol| {
            let ctl_cmd_resp = ctl_cmd.execute(core_sol)?;
            let cmd_resps = CmdResps::with_resp(ctl_cmd_resp);
            let info_cmd = info_cmd.br_resolve(&cmd_resps)?;
            let sol_info = info_cmd.execute(ctx.sol_id, ctx.src_alias, core_sol);
            let ctl_cmd_resp = cmd_resps.into_iter().next().unwrap();
            Ok((ctl_cmd_resp, sol_info))
        })
        .await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SolChangeEnumSolInfoError {
    #[error(transparent)]
    Change(#[from] SolChangeEnumError),
    #[error("failed to resolve backref in info command")]
    InfoBrResolve(#[from] BrResolveError),
}
