use std::num::Wrapping;

use crate::util::{LibDefault, LibIncrement};

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, derive_more::Display)]
pub struct FleetId(u32);
impl LibDefault for FleetId {
    fn lib_default() -> Self {
        Self(0)
    }
}
impl LibIncrement for FleetId {
    fn lib_increment(&mut self) {
        self.0 = (Wrapping(self.0) + Wrapping(1)).0;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Error
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(thiserror::Error, Debug)]
#[error("fleet {fleet_id} not found")]
pub struct FleetFoundError {
    pub fleet_id: FleetId,
}
// Conversion needed for unified user entity container to work
impl From<FleetId> for FleetFoundError {
    fn from(fleet_id: FleetId) -> Self {
        Self { fleet_id }
    }
}
