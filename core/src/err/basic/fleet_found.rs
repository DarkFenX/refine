use crate::ud::FleetId;

#[derive(thiserror::Error, Debug)]
#[error("fleet {fleet_id} not found")]
pub struct FleetFoundError {
    pub fleet_id: FleetId,
}
