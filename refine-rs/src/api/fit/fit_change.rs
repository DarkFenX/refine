use crate::{
    ChangeFitEnumCmd, CtlCmdResps, Fit, FitInfo, FitInfoArgsBackref,
    err::{BackrefRenderError, ChangeFitEnumError},
    info::{FitInfoModesInt, ItemInfoModesInt},
};

impl Fit<'_, '_> {
    #[tracing::instrument(name = "fit-chg", level = "trace", skip_all)]
    pub async fn change(&mut self, ctl_cmds: Vec<ChangeFitEnumCmd>) -> Result<CtlCmdResps, ChangeFitError> {
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
    #[tracing::instrument(name = "fit-chg-inf", level = "trace", skip_all)]
    pub async fn change_and_get_info(
        &mut self,
        ctl_cmds: Vec<ChangeFitEnumCmd>,
        info_args: FitInfoArgsBackref,
    ) -> Result<(CtlCmdResps, FitInfo), ChangeFitError> {
        // Variables for move
        let fit_id = self.id;
        self.sol
            .exec_standard_fallible(move |core_sol| {
                // Holding mutex on sol - nothing can remove the core fit without consuming the
                // high-level Fit
                let mut core_fit = core_sol.get_fit_mut(&fit_id).unwrap();
                let ctl_cmd_resps = execute_commands(&mut core_fit, ctl_cmds)?;
                let fit_info_modes = FitInfoModesInt::from_pub_mode(info_args.fit);
                let item_info_modes = ItemInfoModesInt::from_pub_modes_backref(info_args.item, &ctl_cmd_resps);
                let fit_info = FitInfo::from_core(&mut core_fit, &fit_info_modes, &item_info_modes);
                Ok((ctl_cmd_resps, fit_info))
            })
            .await
    }
}

fn execute_commands(core_fit: &mut rc::FitMut, ctl_cmds: Vec<ChangeFitEnumCmd>) -> Result<CtlCmdResps, ChangeFitError> {
    let mut ctl_cmd_resps = CtlCmdResps::with_capacity(ctl_cmds.len());
    for (index, ctl_cmd) in ctl_cmds.into_iter().enumerate() {
        let ctl_cmd_resp = ctl_cmd
            .render(&ctl_cmd_resps)
            .map_err(|render_err| ChangeFitError::from_render(index, render_err))?
            .execute(core_fit)
            .map_err(|exec_err| ChangeFitError::from_exec(index, exec_err))?;
        ctl_cmd_resps.append(ctl_cmd_resp);
    }
    Ok(ctl_cmd_resps)
}

#[derive(Debug, thiserror::Error)]
pub enum ChangeFitError {
    #[error("command #{0} failed")]
    CtlRender(usize, #[source] BackrefRenderError),
    #[error("command #{0} failed")]
    CtlExec(usize, #[source] ChangeFitEnumError),
}
impl ChangeFitError {
    fn from_render(cmd_idx: usize, render_err: BackrefRenderError) -> Self {
        Self::CtlRender(cmd_idx, render_err)
    }
    fn from_exec(cmd_idx: usize, exec_err: ChangeFitEnumError) -> Self {
        Self::CtlExec(cmd_idx, exec_err)
    }
}
