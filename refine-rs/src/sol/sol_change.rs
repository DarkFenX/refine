use crate::{
    cmd::{BackrefRenderError, ChangeSolEnumCmd, ChangeSolEnumError, CmdResps},
    info::{FitInfoMode, FleetInfoMode, ItemInfoMode, SolInfo, SolInfoMode},
    sol::SolarSystem,
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, cmds: Vec<ChangeSolEnumCmd>) -> Result<CmdResps, ChangeSolError> {
        self.exec_standard_fallible(move |core_sol| {
            let cmd_resps = execute_commands(core_sol, cmds)?;
            Ok(cmd_resps)
        })
        .await
    }
    #[tracing::instrument(name = "sol-chg", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        cmds: Vec<ChangeSolEnumCmd>,
        sol_mode: SolInfoMode,
        fleet_mode: FleetInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Result<(CmdResps, SolInfo), ChangeSolError> {
        let sol_id = self.id;
        self.exec_standard_fallible(move |core_sol| {
            let cmd_resps = execute_commands(core_sol, cmds)?;
            let sol_info = SolInfo::from_id_and_core(sol_id, core_sol, sol_mode, fleet_mode, fit_mode, item_mode);
            Ok((cmd_resps, sol_info))
        })
        .await
    }
}

fn execute_commands(core_sol: &mut rc::SolarSystem, cmds: Vec<ChangeSolEnumCmd>) -> Result<CmdResps, ChangeSolError> {
    let mut cmd_resps = CmdResps::with_capacity(cmds.len());
    for (index, cmd) in cmds.into_iter().enumerate() {
        let resp = cmd
            .render(&cmd_resps)
            .map_err(|render_err| ChangeSolError::from_render(index, render_err))?
            .execute(core_sol)
            .map_err(|exec_err| ChangeSolError::from_exec(index, exec_err))?;
        cmd_resps.append(resp);
    }
    Ok(cmd_resps)
}

#[derive(Debug, thiserror::Error)]
pub enum ChangeSolError {
    #[error("command #{0} failed: {1}")]
    RenderFailed(usize, #[source] BackrefRenderError),
    #[error("command #{0} failed: {1}")]
    ExecFailed(usize, #[source] ChangeSolEnumError),
}
impl ChangeSolError {
    fn from_render(cmd_idx: usize, render_err: BackrefRenderError) -> Self {
        Self::RenderFailed(cmd_idx, render_err)
    }
    fn from_exec(cmd_idx: usize, exec_err: ChangeSolEnumError) -> Self {
        Self::ExecFailed(cmd_idx, exec_err)
    }
}
