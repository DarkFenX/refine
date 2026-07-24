use crate::ud::FitId;

#[derive(Debug, thiserror::Error)]
#[error("fit {fit_id} does not belong to any fleet")]
pub struct FitFleetAssignedError {
    pub fit_id: FitId,
}
