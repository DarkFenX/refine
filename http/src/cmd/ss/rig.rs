use crate::cmd::{fit, item};

#[derive(serde::Deserialize)]
pub(crate) struct HAddRigCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddRigCmd,
}
impl HAddRigCmd {
    pub(in crate::cmd::ss) fn from_fit_cmd(fit_id: rc::SsFitId, fit_cmd: fit::HAddRigCmd) -> Self {
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
pub(crate) struct HChangeRigCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeRigCmd,
}
impl HChangeRigCmd {
    pub(in crate::cmd::ss) fn from_item_cmd(item_id: rc::SsItemId, item_cmd: item::HChangeRigCmd) -> Self {
        Self {
            fit_cmd: fit::HChangeRigCmd::from_item_cmd(item_id, item_cmd),
        }
    }
    pub(crate) fn get_item_id(&self) -> rc::SsItemId {
        self.fit_cmd.get_item_id()
    }
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.fit_cmd.get_state()
    }
}
impl From<fit::HChangeRigCmd> for HChangeRigCmd {
    fn from(fit_cmd: fit::HChangeRigCmd) -> Self {
        Self { fit_cmd }
    }
}
