use crate::{EntityIter, Item, api::ItemSealed};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Item<'s> {
    /// Iterates over item's sub-items, like module's charge, or fighter's autocharges.
    pub fn iter_subitems(&mut self) -> EntityIter<'_, Item<'_>> {
        let item_uid = self.get_uid();
        let core = self.get_core_mut();
        let sub_item_uids = core.u_data.items.get(item_uid).iter_charges().collect();
        EntityIter::new(core, sub_item_uids)
    }
}
