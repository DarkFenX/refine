pub(crate) use booster::HChangeBoosterCmd;
pub(crate) use character::HChangeCharacterCmd;
pub(crate) use drone::HChangeDroneCmd;
pub(crate) use fighter::HChangeFighterCmd;
pub(crate) use implant::HChangeImplantCmd;
pub(crate) use module::HChangeModuleCmd;
pub(crate) use rig::HChangeRigCmd;

mod booster;
mod character;
mod drone;
mod fighter;
mod implant;
mod module;
mod rig;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemCommand {
    ChangeCharacter(HChangeCharacterCmd),
    ChangeImplant(HChangeImplantCmd),
    ChangeBooster(HChangeBoosterCmd),
    ChangeModule(HChangeModuleCmd),
    ChangeRig(HChangeRigCmd),
    ChangeDrone(HChangeDroneCmd),
    ChangeFighter(HChangeFighterCmd),
}
