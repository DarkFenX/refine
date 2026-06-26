use serde::Deserialize;

use crate::{
    cmd::{
        HAddFitCmd, HAddFleetCmd, HCmdResp,
        old_change_sol::{
            HChangeFitCmd, HChangeFleetCmd, HChangeSolCmd, HDeleteFitCmd, HDeleteFleetCmd, HRemoveItemCmd,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeSolCommand {
    // Solar system
    ChangeSol(HChangeSolCmd),
    // Fleet
    AddFleet(HAddFleetCmd),
    ChangeFleet(HChangeFleetCmd),
    DeleteFleet(HDeleteFleetCmd),
    // Fit
    AddFit(HAddFitCmd),
    ChangeFit(HChangeFitCmd),
    DeleteFit(HDeleteFitCmd),
    // Item
    RemoveItem(HRemoveItemCmd),
}
impl HChangeSolCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        match self {
            // Solar system
            #[allow(clippy::unit_arg)]
            Self::ChangeSol(cmd) => Ok(cmd.execute(core_sol).into()),
            // Fleet
            Self::AddFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::DeleteFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Fit
            Self::AddFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::DeleteFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item
            Self::RemoveItem(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}
