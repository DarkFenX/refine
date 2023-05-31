use crate::cmd::shared::{AddMode, State};

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum SsCommand {
    AddImplant(AddImplantCmd),
    SetShip(SetShipCmd),
    AddModuleHigh(AddModuleCmd),
    AddModuleMid(AddModuleCmd),
    AddModuleLow(AddModuleCmd),
    AddRig(AddRigCmd),
}

#[derive(serde::Deserialize)]
pub(crate) struct AddImplantCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl AddImplantCmd {
    pub(crate) fn new(fit_id: rc::ReeId, type_id: rc::ReeInt, state: Option<bool>) -> Self {
        Self { fit_id, type_id, state }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct SetShipCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl SetShipCmd {
    pub(crate) fn new(fit_id: rc::ReeId, type_id: rc::ReeInt, state: Option<bool>) -> Self {
        Self { fit_id, type_id, state }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct AddModuleCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) add_mode: AddMode,
    pub(crate) module_type_id: rc::ReeInt,
    pub(crate) charge_type_id: Option<rc::ReeInt>,
    pub(crate) state: State,
}
impl AddModuleCmd {
    pub(crate) fn new(
        fit_id: rc::ReeId,
        add_mode: AddMode,
        module_type_id: rc::ReeInt,
        charge_type_id: Option<rc::ReeInt>,
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

#[derive(serde::Deserialize)]
pub(crate) struct AddRigCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl AddRigCmd {
    pub(crate) fn new(fit_id: rc::ReeId, type_id: rc::ReeInt, state: Option<bool>) -> Self {
        Self { fit_id, type_id, state }
    }
}
