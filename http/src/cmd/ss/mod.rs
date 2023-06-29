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
pub(crate) use sw_effect::HAddSwEffectCmd;

use crate::cmd::{HFitCommand, HItemCommand};

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
mod sw_effect;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HSsCommand {
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
    AddSwEffect(HAddSwEffectCmd),
}
impl HSsCommand {
    pub(crate) fn from_fit_cmd(fit_id: rc::SsFitId, fit_cmd: HFitCommand) -> Self {
        match fit_cmd {
            HFitCommand::SetCharacter(fit_cmd) => Self::SetCharacter(HSetCharCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddSkill(fit_cmd) => Self::AddSkill(HAddSkillCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddImplant(fit_cmd) => Self::AddImplant(HAddImplantCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddBooster(fit_cmd) => Self::AddBooster(HAddBoosterCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::ChangeBooster(fit_cmd) => Self::ChangeBooster(HChangeBoosterCmd::from(fit_cmd)),
            HFitCommand::SetShip(fit_cmd) => Self::SetShip(HSetShipCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::SetStance(fit_cmd) => Self::SetStance(HSetStanceCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddSubsystem(fit_cmd) => Self::AddSubsystem(HAddSubsystemCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddModule(fit_cmd) => Self::AddModule(HAddModuleCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::ChangeModule(fit_cmd) => Self::ChangeModule(HChangeModuleCmd::from(fit_cmd)),
            HFitCommand::AddRig(fit_cmd) => Self::AddRig(HAddRigCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddDrone(fit_cmd) => Self::AddDrone(HAddDroneCmd::from_fit_cmd(fit_id, fit_cmd)),
            HFitCommand::AddFighter(fit_cmd) => Self::AddFighter(HAddFighterCmd::from_fit_cmd(fit_id, fit_cmd)),
        }
    }
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
