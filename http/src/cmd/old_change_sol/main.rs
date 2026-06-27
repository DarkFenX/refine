use serde::Deserialize;

use crate::{
    cmd::{HCmdResp, old_change_sol::HChangeSolCmd},
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeSolCommand {
    // Solar system
    ChangeSol(HChangeSolCmd),
}
impl HChangeSolCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HCmdResp, HExecError> {
        match self {
            // Solar system
            #[allow(clippy::unit_arg)]
            Self::ChangeSol(cmd) => Ok(cmd.execute(core_sol).into()),
        }
    }
}
