pub(crate) use booster::HChangeBoosterCmd;
pub(crate) use character::HChangeCharacterCmd;
pub(crate) use drone::HChangeDroneCmd;
pub(crate) use fighter::HChangeFighterCmd;
pub(crate) use module::HChangeModuleCmd;

mod booster;
mod character;
mod drone;
mod fighter;
mod module;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemCommand {
    ChangeCharacter(HChangeCharacterCmd),
    ChangeBooster(HChangeBoosterCmd),
    ChangeModule(HChangeModuleCmd),
    ChangeDrone(HChangeDroneCmd),
    ChangeFighter(HChangeFighterCmd),
}
