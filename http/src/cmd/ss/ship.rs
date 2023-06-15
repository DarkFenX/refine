#[derive(serde::Deserialize)]
pub(crate) struct HSetShipCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HSetShipCmd {
    pub(crate) fn new(fit_id: rc::ReeId, type_id: rc::ReeInt, state: Option<bool>) -> Self {
        Self { fit_id, type_id, state }
    }
}
