use crate::{
    ChangeFitEnumCmd, CmdResps, Fit, FitInfo, FitInfoMode, ItemInfoMode,
    err::{BackrefRenderError, ChangeFitEnumError},
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, cmds: Vec<ChangeFitEnumCmd>) -> Result<CmdResps, ChangeFitError> {
        let fit_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                let cmd_resps = execute_commands(&mut core_fit, cmds)?;
                Ok(cmd_resps)
            })
            .await
    }
    #[tracing::instrument(name = "fit-chg", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        cmds: Vec<ChangeFitEnumCmd>,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Result<(CmdResps, FitInfo), ChangeFitError> {
        let fit_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                let cmd_resps = execute_commands(&mut core_fit, cmds)?;
                let fit_info = FitInfo::from_core(&mut core_fit, fit_mode, item_mode);
                Ok((cmd_resps, fit_info))
            })
            .await
    }
}

fn execute_commands(core_fit: &mut rc::FitMut, cmds: Vec<ChangeFitEnumCmd>) -> Result<CmdResps, ChangeFitError> {
    let mut cmd_resps = CmdResps::with_capacity(cmds.len());
    for (index, cmd) in cmds.into_iter().enumerate() {
        let resp = cmd
            .render(&cmd_resps)
            .map_err(|render_err| ChangeFitError::from_render(index, render_err))?
            .execute(core_fit)
            .map_err(|exec_err| ChangeFitError::from_exec(index, exec_err))?;
        cmd_resps.append(resp);
    }
    Ok(cmd_resps)
}

#[derive(Debug, thiserror::Error)]
pub enum ChangeFitError {
    #[error("command #{0} failed: {1}")]
    RenderFailed(usize, #[source] BackrefRenderError),
    #[error("command #{0} failed: {1}")]
    ExecFailed(usize, #[source] ChangeFitEnumError),
}
impl ChangeFitError {
    fn from_render(cmd_idx: usize, render_err: BackrefRenderError) -> Self {
        Self::RenderFailed(cmd_idx, render_err)
    }
    fn from_exec(cmd_idx: usize, exec_err: ChangeFitEnumError) -> Self {
        Self::ExecFailed(cmd_idx, exec_err)
    }
}
