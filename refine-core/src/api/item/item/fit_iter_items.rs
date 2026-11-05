use crate::{EntityIter, Fit, Item};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    /// Iterates over items stored directly on the fit (e.g. modules, but not their charges).
    pub fn iter_items(&mut self) -> EntityIter<'_, Item<'_>> {
        let item_uids = self.sol.u_data.fits.get(self.uid).iter_direct_items().collect();
        EntityIter::new(self.sol, item_uids)
    }
}
