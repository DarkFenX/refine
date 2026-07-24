use crate::ud::FitId;

#[derive(Clone, Debug, thiserror::Error)]
#[error("fit {fit_id} has no ship")]
pub struct FitHasShipError {
    pub fit_id: FitId,
}
