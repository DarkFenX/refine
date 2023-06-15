use crate::shared::HState;

#[derive(serde::Deserialize)]
pub(crate) struct HAddDroneCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: HState,
}
impl HAddDroneCmd {
    pub(crate) fn new(fit_id: rc::ReeId, type_id: rc::ReeInt, state: HState) -> Self {
        Self { fit_id, type_id, state }
    }
}
