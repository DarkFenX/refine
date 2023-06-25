pub(crate) use character::HSetCharCmd;
pub(crate) use drone::HAddDroneCmd;
pub(crate) use implant::HAddImplantCmd;
pub(crate) use module::HAddModuleCmd;
pub(crate) use rig::HAddRigCmd;
pub(crate) use ship::HSetShipCmd;

use crate::cmd::fit::HFitCommand;

mod character;
mod drone;
mod implant;
mod module;
mod rig;
mod ship;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HSsCommand {
    SetCharacter(HSetCharCmd),
    AddImplant(HAddImplantCmd),
    SetShip(HSetShipCmd),
    AddModule(HAddModuleCmd),
    AddRig(HAddRigCmd),
    AddDrone(HAddDroneCmd),
}
impl HSsCommand {
    pub(crate) fn from_fit_cmd(fit_id: rc::ReeId, fit_cmd: HFitCommand) -> Self {
        match fit_cmd {
            HFitCommand::SetCharacter(fit_cmd) => Self::SetCharacter(HSetCharCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddImplant(fit_cmd) => Self::AddImplant(HAddImplantCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::SetShip(fit_cmd) => Self::SetShip(HSetShipCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddModule(fit_cmd) => Self::AddModule(HAddModuleCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddRig(fit_cmd) => Self::AddRig(HAddRigCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddDrone(fit_cmd) => Self::AddDrone(HAddDroneCmd::from_fit_cmd(fit_id, fit_cmd)),
        }
    }
}
