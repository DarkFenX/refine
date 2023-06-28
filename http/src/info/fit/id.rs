#[derive(serde::Serialize)]
pub(crate) struct HFitInfoId {
    pub(crate) id: String,
}
impl From<&rc::SsFitId> for HFitInfoId {
    fn from(fit_id: &rc::SsFitId) -> Self {
        Self { id: fit_id.to_string() }
    }
}
