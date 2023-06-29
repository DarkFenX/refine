pub(crate) use booster::{HAddBoosterCmd, HChangeBoosterCmd};
pub(crate) use character::{HChangeCharacterCmd, HSetCharacterCmd};
pub(crate) use drone::{HAddDroneCmd, HChangeDroneCmd};
pub(crate) use fighter::{HAddFighterCmd, HChangeFighterCmd};
pub(crate) use implant::{HAddImplantCmd, HChangeImplantCmd};
pub(crate) use module::{HAddModuleCmd, HChangeModuleCmd};
pub(crate) use rig::{HAddRigCmd, HChangeRigCmd};
pub(crate) use ship::{HChangeShipCmd, HSetShipCmd};
pub(crate) use skill::HAddSkillCmd;
pub(crate) use stance::{HChangeStanceCmd, HSetStanceCmd};
pub(crate) use subsystem::HAddSubsystemCmd;

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
    SetCharacter(HSetCharacterCmd),
    ChangeCharacter(HChangeCharacterCmd),
    AddSkill(HAddSkillCmd),
    AddImplant(HAddImplantCmd),
    ChangeImplant(HChangeImplantCmd),
    AddBooster(HAddBoosterCmd),
    ChangeBooster(HChangeBoosterCmd),
    SetShip(HSetShipCmd),
    ChangeShip(HChangeShipCmd),
    SetStance(HSetStanceCmd),
    ChangeStance(HChangeStanceCmd),
    AddSubsystem(HAddSubsystemCmd),
    AddModule(HAddModuleCmd),
    ChangeModule(HChangeModuleCmd),
    AddRig(HAddRigCmd),
    ChangeRig(HChangeRigCmd),
    AddDrone(HAddDroneCmd),
    ChangeDrone(HChangeDroneCmd),
    AddFighter(HAddFighterCmd),
    ChangeFighter(HChangeFighterCmd),
}
