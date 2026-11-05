use crate::{EntityIter, Item, api::ItemSealed, sol::SolCore, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Item<'s> {
    /// Iterates over item's sub-items, like module's charge, or fighter's autocharges.
    pub fn iter_subitems(&mut self) -> EntityIter<'_, Item<'_>> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let sub_item_uids = sol.internal_get_item_subitems_uid(item_uid);
        EntityIter::new(sol, sub_item_uids)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_get_item_subitems_uid(&self, item_uid: UItemId) -> Vec<UItemId> {
        self.u_data.items.get(item_uid).iter_charges().collect()
    }
}
