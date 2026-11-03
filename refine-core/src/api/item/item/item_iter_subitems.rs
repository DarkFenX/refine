use crate::{EntityIter, Item, api::ItemSealed};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Item<'s> {
    /// Iterates over item's sub-items, like module's charge, or fighter's autocharges.
    pub fn iter_subitems(&mut self) -> EntityIter<'_, Item<'_>> {
        let item_uid = self.get_uid();
        let sol = self.get_sol_mut();
        let sub_item_uids = sol.u_data.items.get(item_uid).iter_charges().collect();
        EntityIter::new(sol, sub_item_uids)
    }
}
