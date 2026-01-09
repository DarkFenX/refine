use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HFitInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::FitId,
}
impl From<&mut rc::FitMut<'_>> for HFitInfoId {
    fn from(core_fit: &mut rc::FitMut<'_>) -> Self {
        Self {
            id: core_fit.get_fit_id(),
        }
    }
}
