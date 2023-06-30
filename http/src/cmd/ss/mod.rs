pub(crate) use booster::{HAddBoosterCmd, HChangeBoosterCmd};
pub(crate) use character::{HChangeCharacterCmd, HSetCharacterCmd};
pub(crate) use drone::{HAddDroneCmd, HChangeDroneCmd};
pub(crate) use fighter::{HAddFighterCmd, HChangeFighterCmd};
pub(crate) use implant::{HAddImplantCmd, HChangeImplantCmd};
pub(crate) use module::{HAddModuleCmd, HChangeModuleCmd};
pub(crate) use rig::{HAddRigCmd, HChangeRigCmd};
pub(crate) use ship::{HChangeShipCmd, HSetShipCmd};
pub(crate) use skill::{HAddSkillCmd, HChangeSkillCmd};
pub(crate) use stance::{HChangeStanceCmd, HSetStanceCmd};
pub(crate) use subsystem::{HAddSubsystemCmd, HChangeSubsystemCmd};
pub(crate) use sw_effect::{HAddSwEffectCmd, HChangeSwEffectCmd};

use crate::cmd::HCmdResp;

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
    SetCharacter(HSetCharacterCmd),
    ChangeCharacter(HChangeCharacterCmd),
    AddSkill(HAddSkillCmd),
    ChangeSkill(HChangeSkillCmd),
    AddImplant(HAddImplantCmd),
    ChangeImplant(HChangeImplantCmd),
    AddBooster(HAddBoosterCmd),
    ChangeBooster(HChangeBoosterCmd),
    SetShip(HSetShipCmd),
    ChangeShip(HChangeShipCmd),
    SetStance(HSetStanceCmd),
    ChangeStance(HChangeStanceCmd),
    AddSubsystem(HAddSubsystemCmd),
    ChangeSubsystem(HChangeSubsystemCmd),
    AddModule(HAddModuleCmd),
    ChangeModule(HChangeModuleCmd),
    AddRig(HAddRigCmd),
    ChangeRig(HChangeRigCmd),
    AddDrone(HAddDroneCmd),
    ChangeDrone(HChangeDroneCmd),
    AddFighter(HAddFighterCmd),
    ChangeFighter(HChangeFighterCmd),
    AddSwEffect(HAddSwEffectCmd),
    ChangeSwEffect(HChangeSwEffectCmd),
}
impl HSsCommand {
    pub(crate) fn execute(&self, core_ss: &mut rc::SolarSystem) -> rc::Result<HCmdResp> {
        match self {
            Self::SetCharacter(cmd) => cmd.execute(core_ss),
            Self::ChangeCharacter(cmd) => cmd.execute(core_ss),
            Self::AddSkill(cmd) => cmd.execute(core_ss),
            Self::ChangeSkill(cmd) => cmd.execute(core_ss),
            Self::AddImplant(cmd) => cmd.execute(core_ss),
            Self::ChangeImplant(cmd) => cmd.execute(core_ss),
            Self::AddBooster(cmd) => cmd.execute(core_ss),
            Self::ChangeBooster(cmd) => cmd.execute(core_ss),
            Self::SetShip(cmd) => cmd.execute(core_ss),
            Self::ChangeShip(cmd) => cmd.execute(core_ss),
            Self::SetStance(cmd) => cmd.execute(core_ss),
            Self::ChangeStance(cmd) => cmd.execute(core_ss),
            Self::AddSubsystem(cmd) => cmd.execute(core_ss),
            Self::ChangeSubsystem(cmd) => cmd.execute(core_ss),
            Self::AddModule(cmd) => cmd.execute(core_ss),
            Self::ChangeModule(cmd) => cmd.execute(core_ss),
            Self::AddRig(cmd) => cmd.execute(core_ss),
            Self::ChangeRig(cmd) => cmd.execute(core_ss),
            Self::AddDrone(cmd) => cmd.execute(core_ss),
            Self::ChangeDrone(cmd) => cmd.execute(core_ss),
            Self::AddFighter(cmd) => cmd.execute(core_ss),
            Self::ChangeFighter(cmd) => cmd.execute(core_ss),
            Self::AddSwEffect(cmd) => cmd.execute(core_ss),
            Self::ChangeSwEffect(cmd) => cmd.execute(core_ss),
        }
    }
}
