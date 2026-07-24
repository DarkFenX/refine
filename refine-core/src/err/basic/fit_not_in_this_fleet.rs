use crate::ud::{FitId, FleetId};

#[derive(Debug, thiserror::Error)]
#[error("fit {fit_id} is already a member of fleet {fleet_id}")]
pub struct FitNotInThisFleetError {
    pub fit_id: FitId,
    pub fleet_id: FleetId,
}
