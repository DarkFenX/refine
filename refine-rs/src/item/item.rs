use crate::sol::SolarSystem;

pub struct Item<'r, 's> {
    pub(super) sol: &'s mut SolarSystem<'r>,
    pub(super) id: rc::ItemId,
}
impl<'r, 's> Item<'r, 's> {
    pub fn get_item_id(&self) -> rc::ItemId {
        self.id
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'r, 's> Item<'r, 's> {
    pub(super) fn new(sol: &'s mut SolarSystem<'r>, id: rc::ItemId) -> Self {
        Self { sol, id }
    }
}
