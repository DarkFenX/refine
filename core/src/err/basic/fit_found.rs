use crate::ud::FitId;

#[derive(thiserror::Error, Debug)]
#[error("fit {fit_id} not found")]
pub struct FitFoundError {
    pub fit_id: FitId,
}
