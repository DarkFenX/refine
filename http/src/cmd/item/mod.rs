pub(crate) use booster::HChangeBoosterCmd;
pub(crate) use module::HChangeModuleCmd;

mod booster;
mod module;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemCommand {
    ChangeBooster(HChangeBoosterCmd),
    ChangeModule(HChangeModuleCmd),
}
