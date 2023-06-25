pub(crate) use module::HChangeModuleCmd;

mod module;

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HItemCommand {
    ChangeModule(HChangeModuleCmd),
}
