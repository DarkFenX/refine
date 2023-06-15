#[derive(serde::Deserialize)]
pub(crate) struct HSetCharCmd {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::ReeId,
    pub(crate) type_id: rc::ReeInt,
    pub(crate) state: Option<bool>,
}
impl HSetCharCmd {
    pub(crate) fn new(fit_id: rc::ReeId, type_id: rc::ReeInt, state: Option<bool>) -> Self {
        Self { fit_id, type_id, state }
    }
}
