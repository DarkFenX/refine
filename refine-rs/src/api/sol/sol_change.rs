use crate::{
    ChangeSolEnumCmd, CtlCmdResps, SolInfo, SolInfoArgsBackref, SolarSystem,
    err::{BackrefRenderError, ChangeSolEnumError},
    info::{FitInfoModesInt, FleetInfoModesInt, ItemInfoModesInt},
};

impl SolarSystem<'_> {
    #[tracing::instrument(name = "sol-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, cmds: Vec<ChangeSolEnumCmd>) -> Result<CtlCmdResps, ChangeSolError> {
        self.exec_standard_fallible(move |core_sol| {
            let cmd_resps = execute_commands(core_sol, cmds)?;
            Ok(cmd_resps)
        })
        .await
    }
    #[tracing::instrument(name = "sol-chg-inf", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        exec_cmds: Vec<ChangeSolEnumCmd>,
        info_args: SolInfoArgsBackref,
    ) -> Result<(CtlCmdResps, SolInfo), ChangeSolError> {
        // Variables for move
        let sol_id = self.get_id();
        let src_alias = self.get_src_alias();
        self.exec_standard_fallible(move |core_sol| {
            let ctl_cmd_resps = execute_commands(core_sol, exec_cmds)?;
            let sol_info_mode = info_args.sol;
            let fleet_info_modes = FleetInfoModesInt::from_pub_modes_backref(info_args.fleet, &ctl_cmd_resps);
            let fit_info_modes = FitInfoModesInt::from_pub_modes_backref(info_args.fit, &ctl_cmd_resps);
            let item_info_modes = ItemInfoModesInt::from_pub_modes_backref(info_args.item, &ctl_cmd_resps);
            let sol_info = SolInfo::from_ids_and_core(
                sol_id,
                src_alias,
                core_sol,
                sol_info_mode,
                &fleet_info_modes,
                &fit_info_modes,
                &item_info_modes,
            );
            Ok((ctl_cmd_resps, sol_info))
        })
        .await
    }
}

fn execute_commands(
    core_sol: &mut rc::SolarSystem,
    cmds: Vec<ChangeSolEnumCmd>,
) -> Result<CtlCmdResps, ChangeSolError> {
    let mut cmd_resps = CtlCmdResps::with_capacity(cmds.len());
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
    #[error("command #{0} failed")]
    Render(usize, #[source] BackrefRenderError),
    #[error("command #{0} failed")]
    Exec(usize, #[source] ChangeSolEnumError),
}
impl ChangeSolError {
    fn from_render(cmd_idx: usize, render_err: BackrefRenderError) -> Self {
        Self::Render(cmd_idx, render_err)
    }
    fn from_exec(cmd_idx: usize, exec_err: ChangeSolEnumError) -> Self {
        Self::Exec(cmd_idx, exec_err)
    }
}
