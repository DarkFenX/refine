#[derive(serde::Serialize)]
pub(crate) struct HFitInfoId {
    pub(crate) id: String,
}
impl From<&rc::ReeId> for HFitInfoId {
    fn from(fit_id: &rc::ReeId) -> Self {
        Self { id: fit_id.to_string() }
    }
}
