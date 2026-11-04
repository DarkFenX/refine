use crate::{EntityIter, Item, Sol, ud::UItem};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    /// Iterates over items stored directly on the sol or its fits (e.g. modules, but not their
    /// charges).
    pub fn iter_items(&mut self) -> EntityIter<'_, Item<'_>> {
        let core = self.inner_mut();
        let item_uids = core
            .u_data
            .items
            .iter()
            .filter(|(_, u_item)| !matches!(u_item, UItem::Charge(..) | UItem::Autocharge(..)))
            .map(|(item_uid, _)| item_uid)
            .collect();
        EntityIter::new(core, item_uids)
    }
}
