use crate::{FleetId, SolarSystem};

pub struct Fleet<'r, 's> {
    pub(super) sol: &'s mut SolarSystem<'r>,
    pub(super) id: FleetId,
}
impl<'r, 's> Fleet<'r, 's> {
    pub fn get_fleet_id(&self) -> FleetId {
        self.id
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'r, 's> Fleet<'r, 's> {
    pub(super) fn new(sol: &'s mut SolarSystem<'r>, id: FleetId) -> Self {
        Self { sol, id }
    }
}
