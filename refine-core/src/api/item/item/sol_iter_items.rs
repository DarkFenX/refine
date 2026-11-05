use crate::{
    EntityIter, Item, Sol,
    sol::SolCore,
    ud::{UItem, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    /// Iterates over items stored directly on the sol or its fits (e.g. modules, but not their
    /// charges).
    pub fn iter_items(&mut self) -> EntityIter<'_, Item<'_>> {
        let sol = self.inner_mut();
        let item_uids = sol.internal_get_sol_items_uid();
        EntityIter::new(sol, item_uids)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_get_sol_items_uid(&self) -> Vec<UItemId> {
        self.u_data
            .items
            .iter()
            .filter(|(_, u_item)| !matches!(u_item, UItem::Charge(..) | UItem::Autocharge(..)))
            .map(|(item_uid, _)| item_uid)
            .collect()
    }
}
