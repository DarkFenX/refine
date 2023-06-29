use crate::cmd::{fit, item};

#[derive(serde::Deserialize)]
pub(crate) struct HSetStanceCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HSetStanceCmd,
}
impl HSetStanceCmd {
    pub(in crate::cmd::ss) fn from_fit_cmd(fit_id: rc::SsFitId, fit_cmd: fit::HSetStanceCmd) -> Self {
        Self { fit_id, fit_cmd }
    }
    pub(crate) fn get_fit_id(&self) -> rc::SsFitId {
        self.fit_id
    }
    pub(crate) fn get_type_id(&self) -> rc::EItemId {
        self.fit_cmd.get_type_id()
    }
    pub(crate) fn get_state(&self) -> bool {
        self.fit_cmd.get_state()
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct HChangeStanceCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeStanceCmd,
}
impl HChangeStanceCmd {
    pub(in crate::cmd::ss) fn from_item_cmd(item_id: rc::SsItemId, item_cmd: item::HChangeStanceCmd) -> Self {
        Self {
            fit_cmd: fit::HChangeStanceCmd::from_item_cmd(item_id, item_cmd),
        }
    }
    pub(crate) fn get_item_id(&self) -> rc::SsItemId {
        self.fit_cmd.get_item_id()
    }
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.fit_cmd.get_state()
    }
}
impl From<fit::HChangeStanceCmd> for HChangeStanceCmd {
    fn from(fit_cmd: fit::HChangeStanceCmd) -> Self {
        Self { fit_cmd }
    }
}
