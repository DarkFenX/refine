use crate::{
    cmd::{
        fit, item,
        shared::{HAddMode, HEffectModeMap},
    },
    shared::{HModRack, HState},
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddModuleCmd,
}
impl HAddModuleCmd {
    pub(in crate::cmd::ss) fn from_fit_cmd(fit_id: rc::SsFitId, fit_cmd: fit::HAddModuleCmd) -> Self {
        Self { fit_id, fit_cmd }
    }
    pub(crate) fn get_fit_id(&self) -> rc::SsFitId {
        self.fit_id
    }
    pub(crate) fn get_rack(&self) -> &HModRack {
        self.fit_cmd.get_rack()
    }
    pub(crate) fn get_add_mode(&self) -> &HAddMode {
        self.fit_cmd.get_add_mode()
    }
    pub(crate) fn get_type_id(&self) -> rc::EItemId {
        self.fit_cmd.get_type_id()
    }
    pub(crate) fn get_state(&self) -> &HState {
        self.fit_cmd.get_state()
    }
    pub(crate) fn get_charge_type_id(&self) -> Option<rc::EItemId> {
        self.fit_cmd.get_charge_type_id()
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeModuleCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeModuleCmd,
}
impl HChangeModuleCmd {
    pub(in crate::cmd::ss) fn from_item_cmd(item_id: rc::SsItemId, item_cmd: item::HChangeModuleCmd) -> Self {
        Self {
            fit_cmd: fit::HChangeModuleCmd::from_item_cmd(item_id, item_cmd),
        }
    }
    pub(crate) fn get_item_id(&self) -> rc::SsItemId {
        self.fit_cmd.get_item_id()
    }
    pub(crate) fn get_state(&self) -> Option<&HState> {
        self.fit_cmd.get_state()
    }
    pub(crate) fn get_effect_modes(&self) -> Option<&HEffectModeMap> {
        self.fit_cmd.get_effect_modes()
    }
}
impl From<fit::HChangeModuleCmd> for HChangeModuleCmd {
    fn from(fit_cmd: fit::HChangeModuleCmd) -> Self {
        Self { fit_cmd }
    }
}
