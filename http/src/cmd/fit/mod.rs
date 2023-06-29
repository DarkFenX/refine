pub(crate) use booster::{HAddBoosterCmd, HChangeBoosterCmd};
pub(crate) use character::HSetCharCmd;
pub(crate) use drone::HAddDroneCmd;
pub(crate) use fighter::HAddFighterCmd;
pub(crate) use implant::HAddImplantCmd;
pub(crate) use module::{HAddModuleCmd, HChangeModuleCmd};
pub(crate) use rig::HAddRigCmd;
pub(crate) use ship::HSetShipCmd;
pub(crate) use skill::HAddSkillCmd;
pub(crate) use stance::HSetStanceCmd;
pub(crate) use subsystem::HAddSubsystemCmd;

use crate::cmd::HItemCommand;

mod booster;
mod character;
mod drone;
mod fighter;
mod implant;
mod module;
mod rig;
mod ship;
mod skill;
mod stance;
mod subsystem;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HFitCommand {
    SetCharacter(HSetCharCmd),
    AddSkill(HAddSkillCmd),
    AddImplant(HAddImplantCmd),
    AddBooster(HAddBoosterCmd),
    ChangeBooster(HChangeBoosterCmd),
    SetShip(HSetShipCmd),
    SetStance(HSetStanceCmd),
    AddSubsystem(HAddSubsystemCmd),
    AddModule(HAddModuleCmd),
    ChangeModule(HChangeModuleCmd),
    AddRig(HAddRigCmd),
    AddDrone(HAddDroneCmd),
    AddFighter(HAddFighterCmd),
}
impl HFitCommand {
    pub(crate) fn from_item_cmd(item_id: rc::SsItemId, item_cmd: HItemCommand) -> Self {
        match item_cmd {
            HItemCommand::ChangeBooster(item_cmd) => {
                Self::ChangeBooster(HChangeBoosterCmd::from_item_cmd(item_id, item_cmd))
            }
            HItemCommand::ChangeModule(item_cmd) => {
                Self::ChangeModule(HChangeModuleCmd::from_item_cmd(item_id, item_cmd))
            }
        }
    }
}
