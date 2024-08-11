use crate::defs::SolFleetId;

#[derive(Debug)]
pub struct FleetFoundError {
    pub fleet_id: SolFleetId,
}
impl FleetFoundError {
    pub(crate) fn new(fleet_id: SolFleetId) -> Self {
        Self { fleet_id }
    }
}
impl std::error::Error for FleetFoundError {}
impl std::fmt::Display for FleetFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fleet {} not found", self.fleet_id)
    }
}
