use crate::{
    CmdResps, SolChangeEnumCmdBr, SolInfo, SolInfoCmdBr, SolarSystem,
    err::{BrResolveError, SolChangeEnumError},
    shared::{CmdResidue, ResidueResolver},
    stats::{SolStatsCmdBr, SolStatsResp},
    val::{SolValCmdBr, SolValResult},
};

impl SolarSystem<'_> {
    /// Apply changes requested by control commands and run validation of the solar system. After
    /// validation is complete, the evaluator function is called, which decides fate of the
    /// transaction: if it returns an error, solar system state before any changes is restored, and
    /// if it does not, info and stats are generated, and all the info is returned.
    #[tracing::instrument(name = "sol-app", level = "trace", skip_all)]
    pub async fn fitting_app_batch<F, E>(
        &mut self,
        ctl_cmds: Vec<SolChangeEnumCmdBr>,
        val_cmd: SolValCmdBr,
        evaluator: F,
        info_cmd: SolInfoCmdBr,
        stats_cmd: SolStatsCmdBr,
    ) -> Result<SolFittingAppResp, SolFittingAppError<E>>
    where
        F: FnOnce(SolValResult) -> Result<SolValResult, E> + Send + Sync + 'static,
        E: std::error::Error + Send + Sync + 'static,
    {
        let sol_backup = ResidueResolver::new().add_cmds(
            ctl_cmds
                .iter()
                .map(|ctl_cmd| ctl_cmd.exec_residue())
                .chain([val_cmd.exec_residue()])
                // Evaluator does not modify solar system, but can fail
                .chain([CmdResidue::ImmutFallible])
                .chain([info_cmd.exec_residue(), stats_cmd.exec_residue()]),
        );
        // Variables for move
        let sol_ctx = self.get_ctx();
        self.exec_standard(sol_backup, move |core_sol| {
            let mut ctl_cmd_resps = CmdResps::with_capacity(ctl_cmds.len());
            for (index, ctl_cmd) in ctl_cmds.into_iter().enumerate() {
                let ctl_cmd_resp = ctl_cmd
                    .br_resolve(&ctl_cmd_resps)
                    .map_err(|br_err| SolFittingAppError::from_ctl_br_resolve(index, br_err))?
                    .execute(core_sol)
                    .map_err(|exec_err| SolFittingAppError::from_ctl_exec(index, exec_err))?;
                ctl_cmd_resps.append(ctl_cmd_resp);
            }
            let val_result = val_cmd.br_resolve(&ctl_cmd_resps).execute(core_sol);
            let val_result = evaluator(val_result).map_err(SolFittingAppError::Evaluator)?;
            let info = info_cmd
                .br_resolve(&ctl_cmd_resps)
                .execute(sol_ctx.sol_id, sol_ctx.src_alias, core_sol);
            let stats = stats_cmd.br_resolve(&ctl_cmd_resps).execute(core_sol);
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
pub enum SolFittingAppError<E>
where
    E: std::error::Error,
{
    #[error("control command #{0} failed")]
    CtlBrResolve(usize, #[source] BrResolveError),
    #[error("control command #{0} failed")]
    CtlExec(usize, #[source] SolChangeEnumError),
    #[error("evaluator failed")]
    Evaluator(#[source] E),
}
impl<E> SolFittingAppError<E>
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
