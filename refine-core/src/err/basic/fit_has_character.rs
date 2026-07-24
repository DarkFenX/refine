use crate::ud::FitId;

#[derive(Clone, Debug, thiserror::Error)]
#[error("fit {fit_id} has no character")]
pub struct FitHasCharacterError {
    pub fit_id: FitId,
}
