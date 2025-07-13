use crate::def::FitId;

#[derive(thiserror::Error, Debug)]
#[error("fit {fit_id} has no ship")]
pub struct FitHasShipError {
    pub fit_id: FitId,
}
