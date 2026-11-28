use crate::{
    CmdResps, Sol,
    app::StatsCmdBuilderInput,
    ctl::{SolChangeEnumCmdBr, err::SolChangeEnumError},
    err::BrResolveError,
    info::{SolInfo, SolInfoCmdBr},
    stats::{SolStatsCmd, SolStatsResp},
    val::{SolValCmdBr, SolValResult},
};

impl Sol<'_> {
    /// Apply changes requested by control commands, run validation of the solar system, generate
    /// info and stats, and return all of it.
    ///
    /// If any of control commands fails, solar system state before any changes is restored.
    ///
    /// Stats command is built by the passed function once the changes are applied and info is
    /// generated, which lets it request stats for items the caller could not see in advance.
    #[tracing::instrument(name = "sol-app", level = "error", skip_all)]
    pub async fn fitting_app_simple_batch<SB>(
        &mut self,
        ctl_cmds: Vec<SolChangeEnumCmdBr>,
        val_cmd: SolValCmdBr,
        info_cmd: SolInfoCmdBr,
        stats_cmd_builder: SB,
    ) -> Result<SolFittingAppSimpleResp, SolFittingAppSimpleError>
    where
        SB: FnOnce(StatsCmdBuilderInput<'_>) -> SolStatsCmd + Send + 'static,
    {
        // Variables for move
        let sol_ctx = self.get_ctx();
        self.exec_standard(move |core_sol| {
            // Execute received control commands
            let mut ctl_cmd_resps = CmdResps::with_capacity(ctl_cmds.len());
            for (index, ctl_cmd) in ctl_cmds.into_iter().enumerate() {
                let ctl_cmd_resp = ctl_cmd
                    .br_resolve(&ctl_cmd_resps)
                    .map_err(|br_err| SolFittingAppSimpleError::from_ctl_br_resolve(index, br_err))?
                    .execute(core_sol)
                    .map_err(|exec_err| SolFittingAppSimpleError::from_ctl_exec(index, exec_err))?;
                ctl_cmd_resps.append(ctl_cmd_resp);
            }
            let val_result = val_cmd.br_resolve(&ctl_cmd_resps).execute_owned(core_sol);
            let info = info_cmd
                .br_resolve(&ctl_cmd_resps)
                .execute(sol_ctx.sol_id, sol_ctx.src_alias, core_sol);
            let stats_cmd = stats_cmd_builder(StatsCmdBuilderInput {
                ctl: &ctl_cmd_resps,
                ctl_fix: &CmdResps::new(),
                info: &info,
            });
            let stats = stats_cmd.execute(core_sol);
            Ok(SolFittingAppSimpleResp {
                ctl: ctl_cmd_resps,
                val: val_result,
                info,
                stats,
            })
        })
        .await
    }
}

pub struct SolFittingAppSimpleResp {
    pub ctl: CmdResps,
    pub val: SolValResult,
    pub info: SolInfo,
    pub stats: SolStatsResp,
}

#[derive(Debug, thiserror::Error)]
pub enum SolFittingAppSimpleError {
    #[error("control command #{0} failed")]
    CtlBrResolve(usize, #[source] BrResolveError),
    #[error("control command #{0} failed")]
    CtlExec(usize, #[source] SolChangeEnumError),
}
impl SolFittingAppSimpleError {
    fn from_ctl_br_resolve(cmd_idx: usize, br_err: BrResolveError) -> Self {
        Self::CtlBrResolve(cmd_idx, br_err)
    }
    fn from_ctl_exec(cmd_idx: usize, ctl_err: SolChangeEnumError) -> Self {
        Self::CtlExec(cmd_idx, ctl_err)
    }
}
