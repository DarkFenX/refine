use crate::cmd::fit;

#[derive(serde::Deserialize)]
pub(crate) struct HAddBoosterCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddBoosterCmd,
}
impl HAddBoosterCmd {
    pub(in crate::cmd::ss) fn from_fit_cmd(fit_id: rc::SsFitId, fit_cmd: fit::HAddBoosterCmd) -> Self {
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
pub(crate) struct HChangeBoosterCmd {
    #[serde(flatten)]
    fit_cmd: fit::HChangeBoosterCmd,
}
impl HChangeBoosterCmd {
    pub(crate) fn get_item_id(&self) -> rc::SsItemId {
        self.fit_cmd.get_item_id()
    }
    pub(crate) fn get_state(&self) -> Option<bool> {
        self.fit_cmd.get_state()
    }
}
impl From<fit::HChangeBoosterCmd> for HChangeBoosterCmd {
    fn from(fit_cmd: fit::HChangeBoosterCmd) -> Self {
        Self { fit_cmd }
    }
}
