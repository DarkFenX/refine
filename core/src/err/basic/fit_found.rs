use crate::def::FitId;

#[derive(thiserror::Error, Debug)]
#[error("fit {fit_id} not found")]
pub struct FitFoundError {
    pub fit_id: FitId,
}
impl From<FitId> for FitFoundError {
    fn from(fit_id: FitId) -> Self {
        Self { fit_id }
    }
}
