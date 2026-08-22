use crate::{
    CmdResps, SolChangeEnumCmdBr, SolInfo, SolInfoCmdBr, SolarSystem,
    err::{BrResolveError, SolChangeEnumError},
    stats::{SolStatsCmdBr, SolStatsResp},
    val::{SolValCmdBr, SolValResult},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-app", level = "trace", skip_all)]
    pub async fn fitting_app_batch<F, E>(
        &mut self,
        ctl_cmds: Vec<SolChangeEnumCmdBr>,
        val_cmd: SolValCmdBr,
        evaluator: F,
        info_cmd: SolInfoCmdBr,
        stats_cmd: SolStatsCmdBr,
    ) -> Result<SolFittingAppResp, SolFittingAppBatchError<E>>
    where
        F: FnOnce(SolValResult) -> Result<SolValResult, E> + Send + 'static,
        E: std::error::Error + Send + 'static,
    {
        self.exec_standard_fallible_ctx(|sol_ctx, core_sol| {
            let mut ctl_cmd_resps = CmdResps::with_capacity(ctl_cmds.len());
            for (index, ctl_cmd) in ctl_cmds.into_iter().enumerate() {
                let ctl_cmd_resp = ctl_cmd
                    .br_resolve(&ctl_cmd_resps)
                    .map_err(|br_err| SolFittingAppBatchError::from_ctl_br_resolve(index, br_err))?
                    .execute(core_sol)
                    .map_err(|exec_err| SolFittingAppBatchError::from_ctl_exec(index, exec_err))?;
                ctl_cmd_resps.append(ctl_cmd_resp);
            }
            let val_result = val_cmd
                .br_resolve(&ctl_cmd_resps)
                .map_err(SolFittingAppBatchError::ValBrResolve)?
                .execute(core_sol);
            let val_result = evaluator(val_result).map_err(SolFittingAppBatchError::Evaluator)?;
            let info = info_cmd
                .br_resolve(&ctl_cmd_resps)
                .map_err(SolFittingAppBatchError::InfoBrResolve)?
                .execute(sol_ctx.sol_id, sol_ctx.src_alias, core_sol);
            let stats = stats_cmd
                .br_resolve(&ctl_cmd_resps)
                .map_err(SolFittingAppBatchError::StatsBrResolve)?
                .execute(core_sol);
            Ok(SolFittingAppResp {
                ctl: ctl_cmd_resps,
                val: val_result,
                info,
                stats,
            })
        })
        .await
    }
}

pub struct SolFittingAppResp {
    pub ctl: CmdResps,
    pub val: SolValResult,
    pub info: SolInfo,
    pub stats: SolStatsResp,
}

#[derive(Debug, thiserror::Error)]
pub enum SolFittingAppBatchError<E>
where
    E: std::error::Error,
{
    #[error("control command #{0} failed")]
    CtlBrResolve(usize, #[source] BrResolveError),
    #[error("control command #{0} failed")]
    CtlExec(usize, #[source] SolChangeEnumError),
    #[error("validation command failed")]
    ValBrResolve(#[source] BrResolveError),
    #[error("evaluator failed")]
    Evaluator(#[source] E),
    #[error("info command failed")]
    InfoBrResolve(#[source] BrResolveError),
    #[error("stats command failed")]
    StatsBrResolve(#[source] BrResolveError),
}
impl<E> SolFittingAppBatchError<E>
where
    E: std::error::Error,
{
    fn from_ctl_br_resolve(cmd_idx: usize, br_err: BrResolveError) -> Self {
        Self::CtlBrResolve(cmd_idx, br_err)
    }
    fn from_ctl_exec(cmd_idx: usize, ctl_err: SolChangeEnumError) -> Self {
        Self::CtlExec(cmd_idx, ctl_err)
    }
}
