use crate::cmd::fit;

#[derive(serde::Deserialize)]
pub(crate) struct HAddImplantCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddImplantCmd,
}
impl HAddImplantCmd {
    pub(in crate::cmd::ss) fn from_fit_cmd(fit_id: rc::SsFitId, fit_cmd: fit::HAddImplantCmd) -> Self {
        Self { fit_id, fit_cmd }
    }
    pub(crate) fn get_fit_id(&self) -> rc::SsFitId {
        self.fit_id
    }
    pub(crate) fn get_type_id(&self) -> rc::ItemId {
        self.fit_cmd.get_type_id()
    }
    pub(crate) fn get_state(&self) -> bool {
        self.fit_cmd.get_state()
    }
}
