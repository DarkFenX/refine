use crate::{
    ItemId,
    err::FitItemGetError,
    stats::{FitStats, ItemStats},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FitStats<'s> {
    pub fn get_item(&mut self, item_id: &ItemId) -> Result<ItemStats<'_>, FitItemGetError> {
        let item_uid = self.sol.internal_get_fit_item_uid(self.uid, item_id)?;
        Ok(ItemStats::new(self.sol, item_uid))
    }
}
