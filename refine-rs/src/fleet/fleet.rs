use crate::sol::SolarSystem;

pub struct Fleet<'r, 's> {
    pub(super) sol: &'s mut SolarSystem<'r>,
    pub(super) id: rc::FleetId,
}
impl<'r, 's> Fleet<'r, 's> {
    pub fn get_fleet_id(&self) -> rc::FleetId {
        self.id
    }
}
// Private part
impl<'r, 's> Fleet<'r, 's> {
    pub(super) fn new(sol: &'s mut SolarSystem<'r>, id: rc::FleetId) -> Self {
        Self { sol, id }
    }
}
