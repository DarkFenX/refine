use crate::{ItemId, SolarSystem};

pub struct Item<'r, 's> {
    pub(super) sol: &'s mut SolarSystem<'r>,
    pub(super) id: ItemId,
}
impl<'r, 's> Item<'r, 's> {
    pub fn get_item_id(&self) -> ItemId {
        self.id
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'r, 's> Item<'r, 's> {
    pub(super) fn new(sol: &'s mut SolarSystem<'r>, id: ItemId) -> Self {
        Self { sol, id }
    }
}
