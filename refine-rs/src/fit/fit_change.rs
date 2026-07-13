use crate::{
    cmd::{BackrefRenderError, ChangeFitEnumCmd, ChangeFitEnumError, CmdResps},
    fit::Fit,
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, cmds: Vec<ChangeFitEnumCmd>) -> Result<CmdResps, ChangeFitError> {
        let fit_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                let mut cmd_resps = CmdResps::with_capacity(cmds.len());
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                for (index, cmd) in cmds.into_iter().enumerate() {
                    let resp = cmd
                        .render(&cmd_resps)
                        .map_err(|render_err| ChangeFitError::from_render(index, render_err))?
                        .execute(&mut core_fit)
                        .map_err(|exec_err| ChangeFitError::from_exec(index, exec_err))?;
                    cmd_resps.append(resp);
                }
                Ok(cmd_resps)
            })
            .await
    }
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
