use crate::sol::FleetId;

#[derive(Debug)]
pub struct FleetFoundError {
    pub fleet_id: FleetId,
}
impl std::error::Error for FleetFoundError {}
impl std::fmt::Display for FleetFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fleet {} not found", self.fleet_id)
    }
}
