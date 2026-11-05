use crate::{
    EntityIter,
    stats::{FitStats, ItemStats},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FitStats<'s> {
    /// Iterates over items stored directly on the fit (e.g. modules, but not their charges).
    pub fn iter_items(&mut self) -> EntityIter<'_, ItemStats<'_>> {
        let item_uids = self.sol.internal_get_fit_items_uid(self.uid);
        EntityIter::new(self.sol, item_uids)
    }
}
