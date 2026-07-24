use crate::ud::{FitId, FleetId};

#[derive(Debug, thiserror::Error)]
#[error("fit {fit_id} is not a member of fleet {fleet_id}")]
pub struct FitInThisFleetError {
    pub fit_id: FitId,
    pub fleet_id: FleetId,
}
