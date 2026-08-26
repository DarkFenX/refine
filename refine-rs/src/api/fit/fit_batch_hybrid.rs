use crate::{
    CmdResps, Fit, FitHybridCmdBr,
    err::{BrResolveError, FitChangeEnumError, FitHybridError, FitInfoEnumError},
    shared::SolBackup,
    stats::err::FitStatsEnumError,
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-hyb", level = "trace", skip_all)]
    pub async fn hybrid_batch(&mut self, cmds: Vec<FitHybridCmdBr>) -> Result<CmdResps, FitHybridBatchError> {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard(SolBackup::Needed, move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                let mut cmd_resps = CmdResps::with_capacity(cmds.len());
                for (index, cmd) in cmds.into_iter().enumerate() {
                    let cmd_resp = cmd
                        .br_resolve(&cmd_resps)
                        .map_err(|br_err| FitHybridBatchError::from_br_resolve(index, br_err))?
                        .execute(&mut core_fit)
                        .map_err(|exec_err| FitHybridBatchError::from_exec(index, exec_err))?;
                    cmd_resps.append(cmd_resp);
                }
                Ok(cmd_resps)
            })
            .await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FitHybridBatchError {
    #[error("command #{0} failed")]
    BrResolve(usize, #[source] BrResolveError),
    #[error("command #{0} failed")]
    CtlExec(usize, #[source] FitChangeEnumError),
    #[error("command #{0} failed")]
    InfoExec(usize, #[source] FitInfoEnumError),
    #[error("command #{0} failed")]
    StatsExec(usize, #[source] FitStatsEnumError),
}
impl FitHybridBatchError {
    fn from_br_resolve(cmd_idx: usize, br_err: BrResolveError) -> Self {
        Self::BrResolve(cmd_idx, br_err)
    }
    fn from_exec(cmd_idx: usize, exec_err: FitHybridError) -> Self {
        match exec_err {
            FitHybridError::Ctl(ctl_err) => Self::CtlExec(cmd_idx, ctl_err),
            FitHybridError::Info(info_err) => Self::InfoExec(cmd_idx, info_err),
            FitHybridError::Stats(stats_err) => Self::StatsExec(cmd_idx, stats_err),
        }
    }
}
