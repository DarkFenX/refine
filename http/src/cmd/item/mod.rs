pub(crate) use booster::HChangeBoosterCmd;
pub(crate) use character::HChangeCharacterCmd;
pub(crate) use drone::HChangeDroneCmd;
pub(crate) use fighter::HChangeFighterCmd;
pub(crate) use implant::HChangeImplantCmd;
pub(crate) use module::HChangeModuleCmd;
pub(crate) use rig::HChangeRigCmd;
pub(crate) use ship::HChangeShipCmd;
pub(crate) use skill::HChangeSkillCmd;
pub(crate) use stance::HChangeStanceCmd;
pub(crate) use subsystem::HChangeSubsystemCmd;
pub(crate) use sw_effect::HChangeSwEffectCmd;

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
pub(crate) enum HItemCommand {
    ChangeCharacter(HChangeCharacterCmd),
    ChangeSkill(HChangeSkillCmd),
    ChangeImplant(HChangeImplantCmd),
    ChangeBooster(HChangeBoosterCmd),
    ChangeShip(HChangeShipCmd),
    ChangeStance(HChangeStanceCmd),
    ChangeSubsystem(HChangeSubsystemCmd),
    ChangeModule(HChangeModuleCmd),
    ChangeRig(HChangeRigCmd),
    ChangeDrone(HChangeDroneCmd),
    ChangeFighter(HChangeFighterCmd),
    ChangeSwEffect(HChangeSwEffectCmd),
}
