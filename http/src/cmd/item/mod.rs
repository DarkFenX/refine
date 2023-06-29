pub(crate) use booster::HChangeBoosterCmd;
pub(crate) use character::HChangeCharacterCmd;
pub(crate) use module::HChangeModuleCmd;

mod booster;
mod character;
mod module;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemCommand {
    ChangeCharacter(HChangeCharacterCmd),
    ChangeBooster(HChangeBoosterCmd),
    ChangeModule(HChangeModuleCmd),
}
