use crate::{
    ChangeSolEnumCmd, CtlCmdResps, SolInfo, SolInfoCmdBackref, SolarSystem,
    err::{BackrefRenderError, ChangeSolEnumError},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, ctl_cmds: Vec<ChangeSolEnumCmd>) -> Result<CtlCmdResps, ChangeSolError> {
        self.exec_standard_fallible(move |core_sol| {
            let ctl_cmd_resps = execute_commands(core_sol, ctl_cmds)?;
            Ok(ctl_cmd_resps)
        })
        .await
    }
    #[tracing::instrument(name = "sol-chg-inf", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        ctl_cmds: Vec<ChangeSolEnumCmd>,
        info_cmd: SolInfoCmdBackref,
    ) -> Result<(CtlCmdResps, SolInfo), ChangeSolError> {
        // Variables for move
        let sol_id = self.get_id();
        let src_alias = self.get_src_alias();
        self.exec_standard_fallible(move |core_sol| {
            let ctl_cmd_resps = execute_commands(core_sol, ctl_cmds)?;
            let sol_info = info_cmd.execute(sol_id, src_alias, core_sol, &ctl_cmd_resps);
            Ok((ctl_cmd_resps, sol_info))
        })
        .await
    }
}

fn execute_commands(
    core_sol: &mut rc::SolarSystem,
    ctl_cmds: Vec<ChangeSolEnumCmd>,
) -> Result<CtlCmdResps, ChangeSolError> {
    let mut ctl_cmd_resps = CtlCmdResps::with_capacity(ctl_cmds.len());
    for (index, ctl_cmd) in ctl_cmds.into_iter().enumerate() {
        let ctl_cmd_resp = ctl_cmd
            .render(&ctl_cmd_resps)
            .map_err(|render_err| ChangeSolError::from_ctl_render(index, render_err))?
            .execute(core_sol)
            .map_err(|exec_err| ChangeSolError::from_ctl_exec(index, exec_err))?;
        ctl_cmd_resps.append(ctl_cmd_resp);
    }
    Ok(ctl_cmd_resps)
}

#[derive(Debug, thiserror::Error)]
pub enum ChangeSolError {
    #[error("command #{0} failed")]
    CtlRender(usize, #[source] BackrefRenderError),
    #[error("command #{0} failed")]
    CtlExec(usize, #[source] ChangeSolEnumError),
}
impl ChangeSolError {
    fn from_ctl_render(cmd_idx: usize, render_err: BackrefRenderError) -> Self {
        Self::CtlRender(cmd_idx, render_err)
    }
    fn from_ctl_exec(cmd_idx: usize, exec_err: ChangeSolEnumError) -> Self {
        Self::CtlExec(cmd_idx, exec_err)
    }
}
