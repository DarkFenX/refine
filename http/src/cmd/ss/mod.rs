pub(crate) use booster::HAddBoosterCmd;
pub(crate) use character::HSetCharCmd;
pub(crate) use drone::HAddDroneCmd;
pub(crate) use fighter::HAddFighterCmd;
pub(crate) use implant::HAddImplantCmd;
pub(crate) use module::{HAddModuleCmd, HChangeModuleCmd};
pub(crate) use rig::HAddRigCmd;
pub(crate) use ship::HSetShipCmd;

use crate::cmd::fit::HFitCommand;

mod booster;
mod character;
mod drone;
mod fighter;
mod implant;
mod module;
mod rig;
mod ship;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HSsCommand {
    SetCharacter(HSetCharCmd),
    AddImplant(HAddImplantCmd),
    AddBooster(HAddBoosterCmd),
    SetShip(HSetShipCmd),
    AddModule(HAddModuleCmd),
    ChangeModule(HChangeModuleCmd),
    AddRig(HAddRigCmd),
    AddDrone(HAddDroneCmd),
    AddFighter(HAddFighterCmd),
}
impl HSsCommand {
    pub(crate) fn from_fit_cmd(fit_id: rc::SsFitId, fit_cmd: HFitCommand) -> Self {
        match fit_cmd {
            HFitCommand::SetCharacter(fit_cmd) => Self::SetCharacter(HSetCharCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddImplant(fit_cmd) => Self::AddImplant(HAddImplantCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddBooster(fit_cmd) => Self::AddBooster(HAddBoosterCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::SetShip(fit_cmd) => Self::SetShip(HSetShipCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddModule(fit_cmd) => Self::AddModule(HAddModuleCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::ChangeModule(fit_cmd) => Self::ChangeModule(HChangeModuleCmd::from(fit_cmd)),
            HFitCommand::AddRig(fit_cmd) => Self::AddRig(HAddRigCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddDrone(fit_cmd) => Self::AddDrone(HAddDroneCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddFighter(fit_cmd) => Self::AddFighter(HAddFighterCmd::from_fit_cmd(fit_id, fit_cmd)),
        }
    }
}
