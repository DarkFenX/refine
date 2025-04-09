use crate::sol::FitId;

#[derive(thiserror::Error, Debug)]
#[error("fit {fit_id} does not belong to any fleet")]
pub struct FitFleetAssignedError {
    pub fit_id: FitId,
}
