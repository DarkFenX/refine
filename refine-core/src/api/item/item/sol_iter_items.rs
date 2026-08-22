use crate::{
    api::{Item, ItemMut, MutIter},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn iter_items(&self) -> impl ExactSizeIterator<Item = Item<'_>> {
        self.u_data.items.keys().map(|item_uid| Item::new(self, item_uid))
    }
    pub fn iter_items_mut(&mut self) -> MutIter<'_, ItemMut<'_>> {
        let item_uids = self.u_data.items.keys().collect();
        MutIter::new(self, item_uids)
    }
}
