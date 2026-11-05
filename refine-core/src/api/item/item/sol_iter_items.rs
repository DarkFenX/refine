use crate::{EntityIter, Item, Sol, ud::UItem};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    /// Iterates over items stored directly on the sol or its fits (e.g. modules, but not their
    /// charges).
    pub fn iter_items(&mut self) -> EntityIter<'_, Item<'_>> {
        let sol = self.inner_mut();
        let item_uids = sol
            .u_data
            .items
            .iter()
            .filter(|(_, u_item)| !matches!(u_item, UItem::Charge(..) | UItem::Autocharge(..)))
            .map(|(item_uid, _)| item_uid)
            .collect();
        EntityIter::new(sol, item_uids)
    }
}
