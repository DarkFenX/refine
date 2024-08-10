use std::{error, fmt};

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
impl error::Error for FleetFoundError {}
impl fmt::Display for FleetFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fleet {} not found", self.fleet_id)
    }
}
