use crate::{
    CmdResps, Fit, FitChangeEnumCmdBr,
    err::{BrResolveError, FitChangeEnumError},
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-bat", level = "trace", skip_all)]
    pub async fn batch(&mut self, ctl_cmds: Vec<FitChangeEnumCmdBr>) -> Result<CmdResps, FitChangeBatchError> {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                let ctl_cmd_resps = execute_commands(&mut core_fit, ctl_cmds)?;
                Ok(ctl_cmd_resps)
            })
            .await
    }
}

fn execute_commands(
    core_fit: &mut rc::FitMut,
    ctl_cmds: Vec<FitChangeEnumCmdBr>,
) -> Result<CmdResps, FitChangeBatchError> {
    let mut ctl_cmd_resps = CmdResps::with_capacity(ctl_cmds.len());
    for (index, ctl_cmd) in ctl_cmds.into_iter().enumerate() {
        let ctl_cmd_resp = ctl_cmd
            .render(&ctl_cmd_resps)
            .map_err(|render_err| FitChangeBatchError::from_ctl_render(index, render_err))?
            .execute(core_fit)
            .map_err(|exec_err| FitChangeBatchError::from_ctl_exec(index, exec_err))?;
        ctl_cmd_resps.append(ctl_cmd_resp);
    }
    Ok(ctl_cmd_resps)
}

#[derive(Debug, thiserror::Error)]
pub enum FitChangeBatchError {
    #[error("command #{0} failed")]
    Render(usize, #[source] BrResolveError),
    #[error("command #{0} failed")]
    CtlExec(usize, #[source] FitChangeEnumError),
}
impl FitChangeBatchError {
    fn from_ctl_render(cmd_idx: usize, render_err: BrResolveError) -> Self {
        Self::Render(cmd_idx, render_err)
    }
    fn from_ctl_exec(cmd_idx: usize, exec_err: FitChangeEnumError) -> Self {
        Self::CtlExec(cmd_idx, exec_err)
    }
}
