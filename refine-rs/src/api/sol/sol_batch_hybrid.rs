use crate::{
    CmdResps, SolHybridCmdBr, SolarSystem,
    err::{BrResolveError, SolChangeEnumError, SolHybridError},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-bat", level = "trace", skip_all)]
    pub async fn hybrid_batch(&mut self, cmds: Vec<SolHybridCmdBr>) -> Result<CmdResps, SolHybridBatchError> {
        self.exec_standard_fallible(move |core_sol| {
            let mut cmd_resps = CmdResps::with_capacity(cmds.len());
            for (index, cmd) in cmds.into_iter().enumerate() {
                let cmd_resp = cmd
                    .br_resolve(&cmd_resps)
                    .map_err(|br_err| SolHybridBatchError::from_br_resolve(index, br_err))?
                    .execute(core_sol)
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
}
impl SolHybridBatchError {
    fn from_br_resolve(cmd_idx: usize, br_err: BrResolveError) -> Self {
        Self::BrResolve(cmd_idx, br_err)
    }
    fn from_exec(cmd_idx: usize, exec_err: SolHybridError) -> Self {
        match exec_err {
            SolHybridError::Ctl(ctl_err) => Self::CtlExec(cmd_idx, ctl_err),
        }
    }
}
