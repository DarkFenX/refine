use crate::{
    EntityIter,
    stats::{ItemStats, SolStats},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> SolStats<'s> {
    /// Iterates over items stored directly on the sol or its fits (e.g. modules, but not their
    /// charges).
    pub fn iter_items(&mut self) -> EntityIter<'_, ItemStats<'_>> {
        let item_uids = self.sol.internal_get_sol_items_uid();
        EntityIter::new(self.sol, item_uids)
    }
}
