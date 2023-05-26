use crate::cmd::shared::{AddMode, State};

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum SsCommand {
    SetShip(SetShipCmd),
    AddModuleHigh(AddModuleCmd),
    AddModuleMid(AddModuleCmd),
    AddModuleLow(AddModuleCmd),
}

#[derive(serde::Deserialize)]
pub(crate) struct SetShipCmd {
    pub(crate) fit_id: reefast::ReeId,
    pub(crate) ship_type_id: reefast::ReeInt,
    pub(crate) state: Option<bool>,
}
impl SetShipCmd {
    pub(crate) fn new(fit_id: reefast::ReeId, ship_type_id: reefast::ReeInt, state: Option<bool>) -> Self {
        Self {
            fit_id,
            ship_type_id,
            state,
        }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct AddModuleCmd {
    pub(crate) fit_id: reefast::ReeId,
    pub(crate) add_mode: AddMode,
    pub(crate) module_type_id: reefast::ReeInt,
    pub(crate) charge_type_id: Option<reefast::ReeInt>,
    pub(crate) state: State,
}
impl AddModuleCmd {
    pub(crate) fn new(
        fit_id: reefast::ReeId,
        add_mode: AddMode,
        module_type_id: reefast::ReeInt,
        charge_type_id: Option<reefast::ReeInt>,
        state: State,
    ) -> Self {
        Self {
            fit_id,
            add_mode,
            module_type_id,
            charge_type_id,
            state,
        }
    }
}
