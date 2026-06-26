use serde::Deserialize;

use crate::{
    cmd::{
        HCmdResp,
        old_change_fit::{
            HChangeCharacterCmd, HChangeFitCmd, HChangeStanceCmd, HSetCharacterCmd,
            HSetStanceCmd, HUnsetCharacterCmd, HUnsetStanceCmd,
        },
    },
    util::HExecError,
};

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HChangeFitCommand {
    // Fit
    ChangeFit(HChangeFitCmd),
    // Item - character
    SetCharacter(HSetCharacterCmd),
    ChangeCharacter(HChangeCharacterCmd),
    UnsetCharacter(HUnsetCharacterCmd),
    // Item - stance
    SetStance(HSetStanceCmd),
    ChangeStance(HChangeStanceCmd),
    UnsetStance(HUnsetStanceCmd),
}
impl HChangeFitCommand {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<HCmdResp, HExecError> {
        match self {
            // Fit
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - character
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::UnsetCharacter(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            // Item - stance
            Self::SetStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::ChangeStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
            Self::UnsetStance(cmd) => Ok(cmd.execute(core_sol, fit_id)?.into()),
        }
    }
}
