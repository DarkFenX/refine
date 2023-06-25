use crate::cmd::fit;

#[derive(serde::Deserialize)]
pub(crate) struct HSetCharCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::ReeId,
    #[serde(flatten)]
    fit_cmd: fit::HSetCharCmd,
}
impl HSetCharCmd {
    pub(in crate::cmd::ss) fn from_fit_cmd(fit_id: rc::ReeId, fit_cmd: fit::HSetCharCmd) -> Self {
        Self { fit_id, fit_cmd }
    }
    pub(crate) fn get_fit_id(&self) -> rc::ReeId {
        self.fit_id
    }
    pub(crate) fn get_type_id(&self) -> rc::ReeInt {
        self.fit_cmd.get_type_id()
    }
    pub(crate) fn get_state(&self) -> bool {
        self.fit_cmd.get_state()
    }
}
