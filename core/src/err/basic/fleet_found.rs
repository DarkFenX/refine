use crate::sol::FleetId;

#[derive(thiserror::Error, Debug)]
#[error("fleet {fleet_id} not found")]
pub struct FleetFoundError {
    pub fleet_id: FleetId,
}
impl From<FleetId> for FleetFoundError {
    fn from(fleet_id: FleetId) -> Self {
        Self { fleet_id }
    }
}
