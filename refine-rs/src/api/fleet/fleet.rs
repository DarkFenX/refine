use crate::{FleetId, Sol};

pub struct Fleet<'r, 's> {
    pub(super) sol: &'s mut Sol<'r>,
    pub(super) id: FleetId,
}
impl<'r, 's> Fleet<'r, 's> {
    pub fn get_id(&self) -> FleetId {
        self.id
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'r, 's> Fleet<'r, 's> {
    pub(super) fn new(sol: &'s mut Sol<'r>, id: FleetId) -> Self {
        Self { sol, id }
    }
}
