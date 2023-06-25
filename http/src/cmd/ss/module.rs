use crate::{
    cmd::{fit, shared::HAddMode},
    shared::{HModRack, HState},
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddModuleCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::ReeId,
    #[serde(flatten)]
    fit_cmd: fit::HAddModuleCmd,
}
impl HAddModuleCmd {
    pub(in crate::cmd::ss) fn from_fit_cmd(fit_id: rc::ReeId, fit_cmd: fit::HAddModuleCmd) -> Self {
        Self { fit_id, fit_cmd }
    }
    pub(crate) fn get_fit_id(&self) -> rc::ReeId {
        self.fit_id
    }
    pub(crate) fn get_rack(&self) -> &HModRack {
        self.fit_cmd.get_rack()
    }
    pub(crate) fn get_add_mode(&self) -> &HAddMode {
        self.fit_cmd.get_add_mode()
    }
    pub(crate) fn get_type_id(&self) -> rc::ReeInt {
        self.fit_cmd.get_type_id()
    }
    pub(crate) fn get_state(&self) -> &HState {
        self.fit_cmd.get_state()
    }
    pub(crate) fn get_charge_type_id(&self) -> Option<rc::ReeInt> {
        self.fit_cmd.get_charge_type_id()
    }
}
