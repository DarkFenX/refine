use crate::ud::FitId;

#[derive(thiserror::Error, Debug)]
#[error("fit {fit_id} has no character")]
pub struct FitHasCharacterError {
    pub fit_id: FitId,
}
