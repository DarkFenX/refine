use crate::cmd::shared::{HAddMode, HState};

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum HSsCommand {
    AddImplant(HAddImplantCmd),
    SetShip(HSetShipCmd),
    AddModuleHigh(HAddModuleCmd),
    AddModuleMid(HAddModuleCmd),
    AddModuleLow(HAddModuleCmd),
    AddRig(HAddRigCmd),
}

#[derive(serde::Deserialize)]
pub(crate) struct HAddImplantCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HAddImplantCmd {
    pub(crate) fn new(fit_id: rc::ReeId, type_id: rc::ReeInt, state: Option<bool>) -> Self {
        Self { fit_id, type_id, state }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HSetShipCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HSetShipCmd {
    pub(crate) fn new(fit_id: rc::ReeId, type_id: rc::ReeInt, state: Option<bool>) -> Self {
        Self { fit_id, type_id, state }
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) add_mode: HAddMode,
    pub(crate) module_type_id: rc::ReeInt,
    pub(crate) charge_type_id: Option<rc::ReeInt>,
    pub(crate) state: HState,
}
impl HAddModuleCmd {
    pub(crate) fn new(
        fit_id: rc::ReeId,
        add_mode: HAddMode,
        module_type_id: rc::ReeInt,
        charge_type_id: Option<rc::ReeInt>,
        state: HState,
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
pub(crate) struct HAddRigCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HAddRigCmd {
    pub(crate) fn new(fit_id: rc::ReeId, type_id: rc::ReeInt, state: Option<bool>) -> Self {
        Self { fit_id, type_id, state }
    }
}
