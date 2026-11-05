use crate::{
    ItemId,
    err::FleetItemGetError,
    stats::{FleetStats, ItemStats},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FleetStats<'s> {
    pub fn get_item(&mut self, item_id: &ItemId) -> Result<ItemStats<'_>, FleetItemGetError> {
        let item_uid = self.sol.internal_get_fleet_item_uid(self.uid, item_id)?;
        Ok(ItemStats::new(self.sol, item_uid))
    }
}
