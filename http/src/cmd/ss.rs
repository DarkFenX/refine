use crate::cmd::shared::{AddMode, State};

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum SsCommand {
    SetShip(SetShipCmd),
    AddModuleHigh(AddModule),
    AddModuleMid(AddModule),
    AddModuleLow(AddModule),
}

#[derive(serde::Deserialize)]
pub(crate) struct SetShipCmd {
    pub(crate) fit_id: reefast::ReeId,
    pub(crate) ship_type_id: reefast::ReeInt,
    pub(crate) state: Option<bool>,
}

#[derive(serde::Deserialize)]
pub(crate) struct AddModule {
    pub(crate) fit_id: reefast::ReeId,
    pub(crate) add_mode: AddMode,
    pub(crate) module_type_id: reefast::ReeInt,
    pub(crate) charge_type_id: Option<reefast::ReeInt>,
    pub(crate) state: State,
}
