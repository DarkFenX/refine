use crate::{
    CmdResps, SolHybridCmdBr, SolarSystem,
    err::{BrResolveError, SolChangeEnumError, SolHybridError, SolInfoEnumError},
    shared::SolBackup,
    stats::err::SolStatsEnumError,
    trial::err::SolTryItemsEnumError,
    val::err::SolValEnumError,
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-hyb", level = "trace", skip_all)]
    pub async fn hybrid_batch(&mut self, cmds: Vec<SolHybridCmdBr>) -> Result<CmdResps, SolHybridBatchError> {
        let backup = SolBackup::Needed;
        let ctx = self.get_ctx();
        self.exec_standard(backup, move |core_sol| {
            let mut cmd_resps = CmdResps::with_capacity(cmds.len());
            for (index, cmd) in cmds.into_iter().enumerate() {
                let cmd_resp = cmd
                    .br_resolve(&cmd_resps)
                    .map_err(|br_err| SolHybridBatchError::from_br_resolve(index, br_err))?
                    .execute(ctx, core_sol)
                    .map_err(|exec_err| SolHybridBatchError::from_exec(index, exec_err))?;
                cmd_resps.append(cmd_resp);
            }
            Ok(cmd_resps)
        })
        .await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SolHybridBatchError {
    #[error("command #{0} failed")]
    BrResolve(usize, #[source] BrResolveError),
    #[error("command #{0} failed")]
    CtlExec(usize, #[source] SolChangeEnumError),
    #[error("command #{0} failed")]
    InfoExec(usize, #[source] SolInfoEnumError),
    #[error("command #{0} failed")]
    StatsExec(usize, #[source] SolStatsEnumError),
    #[error("command #{0} failed")]
    ValExec(usize, #[source] SolValEnumError),
    #[error("command #{0} failed")]
    TryItemsExec(usize, #[source] SolTryItemsEnumError),
}
impl SolHybridBatchError {
    fn from_br_resolve(cmd_idx: usize, br_err: BrResolveError) -> Self {
        Self::BrResolve(cmd_idx, br_err)
    }
    fn from_exec(cmd_idx: usize, exec_err: SolHybridError) -> Self {
        match exec_err {
            SolHybridError::Ctl(ctl_err) => Self::CtlExec(cmd_idx, ctl_err),
            SolHybridError::Info(info_err) => Self::InfoExec(cmd_idx, info_err),
            SolHybridError::Stats(info_err) => Self::StatsExec(cmd_idx, info_err),
            SolHybridError::Val(val_err) => Self::ValExec(cmd_idx, val_err),
            SolHybridError::TryItems(val_err) => Self::TryItemsExec(cmd_idx, val_err),
        }
    }
}
