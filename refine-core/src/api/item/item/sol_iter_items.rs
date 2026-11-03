use crate::{EntityIter, Item, SolarSystem, ud::UItem};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    /// Iterates over items stored directly on the sol or its fits (e.g. modules, but not their
    /// charges).
    pub fn iter_items(&mut self) -> EntityIter<'_, Item<'_>> {
        let item_uids = self
            .u_data
            .items
            .iter()
            .filter(|(_, u_item)| !matches!(u_item, UItem::Charge(..) | UItem::Autocharge(..)))
            .map(|(item_uid, _)| item_uid)
            .collect();
        EntityIter::new(self, item_uids)
    }
}
