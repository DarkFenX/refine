use crate::{EntityIter, Item, SolarSystem};

impl SolarSystem {
    pub fn iter_items(&mut self) -> EntityIter<'_, Item<'_>> {
        let item_uids = self.u_data.items.keys().collect();
        EntityIter::new(self, item_uids)
    }
}
