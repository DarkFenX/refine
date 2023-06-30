use crate::cmd::{fit, item};

#[derive(serde::Deserialize)]
pub(crate) struct HAddSubsystemCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddSubsystemCmd,
}
impl HAddSubsystemCmd {
    pub(in crate::cmd::ss) fn from_fit_cmd(fit_id: rc::SsFitId, fit_cmd: fit::HAddSubsystemCmd) -> Self {
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
pub(crate) struct HChangeSubsystemCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeSubsystemCmd,
}
impl HChangeSubsystemCmd {
    pub(in crate::cmd::ss) fn from_item_cmd(item_id: rc::SsItemId, item_cmd: item::HChangeSubsystemCmd) -> Self {
        Self {
            fit_cmd: fit::HChangeSubsystemCmd::from_item_cmd(item_id, item_cmd),
        }
    }
    pub(crate) fn get_item_id(&self) -> rc::SsItemId {
        self.fit_cmd.get_item_id()
    }
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.fit_cmd.get_state()
    }
}
impl From<fit::HChangeSubsystemCmd> for HChangeSubsystemCmd {
    fn from(fit_cmd: fit::HChangeSubsystemCmd) -> Self {
        Self { fit_cmd }
    }
}
