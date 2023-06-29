use crate::{cmd::fit, shared::HState};

#[derive(serde::Deserialize)]
pub(crate) struct HAddFighterCmd {
    #[serde(with = "crate::util::serde_string")]
    fit_id: rc::SsFitId,
    #[serde(flatten)]
    fit_cmd: fit::HAddFighterCmd,
}
impl HAddFighterCmd {
    pub(in crate::cmd::ss) fn from_fit_cmd(fit_id: rc::SsFitId, fit_cmd: fit::HAddFighterCmd) -> Self {
        Self { fit_id, fit_cmd }
    }
    pub(crate) fn get_fit_id(&self) -> rc::SsFitId {
        self.fit_id
    }
    pub(crate) fn get_type_id(&self) -> rc::EItemId {
        self.fit_cmd.get_type_id()
    }
    pub(crate) fn get_state(&self) -> &HState {
        self.fit_cmd.get_state()
    }
}
