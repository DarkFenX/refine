use crate::{ItemId, Sol};

pub struct Item<'r, 's> {
    pub(super) sol: &'s mut Sol<'r>,
    pub(super) id: ItemId,
}
impl<'r, 's> Item<'r, 's> {
    pub fn get_id(&self) -> ItemId {
        self.id
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'r, 's> Item<'r, 's> {
    pub(super) fn new(sol: &'s mut Sol<'r>, id: ItemId) -> Self {
        Self { sol, id }
    }
}
