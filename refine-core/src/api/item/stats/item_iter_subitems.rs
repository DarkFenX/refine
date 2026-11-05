use crate::{EntityIter, stats::ItemStats};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> ItemStats<'s> {
    /// Iterates over item's sub-items, like module's charge, or fighter's autocharges.
    pub fn iter_subitems(&mut self) -> EntityIter<'_, ItemStats<'_>> {
        let sub_item_uids = self.sol.internal_get_item_subitems_uid(self.uid);
        EntityIter::new(self.sol, sub_item_uids)
    }
}
