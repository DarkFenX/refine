#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFitInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::FitId,
}
impl From<&mut rc::FitMut<'_>> for HFitInfoId {
    fn from(core_fit: &mut rc::FitMut<'_>) -> Self {
        Self {
            id: core_fit.get_fit_id(),
        }
    }
}
