use crate::{
    CmdResps, SolChangeEnumCmdBr, SolInfo, SolInfoCmdBr, SolarSystem,
    err::{BackrefRenderError, SolChangeEnumError},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-bat", level = "trace", skip_all)]
    pub async fn batch(&mut self, ctl_cmds: Vec<SolChangeEnumCmdBr>) -> Result<CmdResps, SolBatchError> {
        self.exec_standard_fallible(move |core_sol| {
            let ctl_cmd_resps = execute_commands(core_sol, ctl_cmds)?;
            Ok(ctl_cmd_resps)
        })
        .await
    }
}

fn execute_commands(
    core_sol: &mut rc::SolarSystem,
    ctl_cmds: Vec<SolChangeEnumCmdBr>,
) -> Result<CmdResps, SolBatchError> {
    let mut ctl_cmd_resps = CmdResps::with_capacity(ctl_cmds.len());
    for (index, ctl_cmd) in ctl_cmds.into_iter().enumerate() {
        let ctl_cmd_resp = ctl_cmd
            .render(&ctl_cmd_resps)
            .map_err(|render_err| SolBatchError::from_ctl_render(index, render_err))?
            .execute(core_sol)
            .map_err(|exec_err| SolBatchError::from_ctl_exec(index, exec_err))?;
        ctl_cmd_resps.append(ctl_cmd_resp);
    }
    Ok(ctl_cmd_resps)
}

#[derive(Debug, thiserror::Error)]
pub enum SolBatchError {
    #[error("command #{0} failed")]
    Render(usize, #[source] BackrefRenderError),
    #[error("command #{0} failed")]
    CtlExec(usize, #[source] SolChangeEnumError),
}
impl SolBatchError {
    fn from_ctl_render(cmd_idx: usize, render_err: BackrefRenderError) -> Self {
        Self::Render(cmd_idx, render_err)
    }
    fn from_ctl_exec(cmd_idx: usize, exec_err: SolChangeEnumError) -> Self {
        Self::CtlExec(cmd_idx, exec_err)
    }
}
