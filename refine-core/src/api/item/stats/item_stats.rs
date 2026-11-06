use crate::{
    sol::{Savepoint, SolCore},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Stats of an item.
pub struct ItemStats<'s> {
    pub(in crate::api) sol: &'s mut SolCore,
    pub(in crate::api) uid: UItemId,
}
/// Stats of an item, with states of the item and items related to it switched up for the item to
/// apply its effects, e.g. modules are enabled, fighters are engaged.
///
/// States are rolled back when it is dropped.
pub struct SwitchedItemStats<'s> {
    pub(in crate::api) sol: &'s mut SolCore,
    pub(in crate::api) uid: UItemId,
    // None when switching changed nothing
    pub(in crate::api) savepoint: Option<Savepoint>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> ItemStats<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}

impl<'s> SwitchedItemStats<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolCore, uid: UItemId) -> Self {
        let savepoint = sol.internal_active_stat_prepare(uid);
        let savepoint = match sol.undo.is_changed_since(&savepoint) {
            true => {
                sol.reuse.cseq_cache_switched.clear();
                Some(savepoint)
            }
            false => {
                sol.internal_tx_rollback(savepoint);
                None
            }
        };
        Self { sol, uid, savepoint }
    }
}
impl Drop for SwitchedItemStats<'_> {
    fn drop(&mut self) {
        if !std::thread::panicking()
            && let Some(savepoint) = self.savepoint.take()
        {
            self.sol.internal_tx_rollback(savepoint);
        }
    }
}
