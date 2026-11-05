use crate::{
    ItemId,
    err::ItemGetError,
    stats::{ItemStats, SolStats},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> SolStats<'s> {
    pub fn get_item(&mut self, item_id: &ItemId) -> Result<ItemStats<'_>, ItemGetError> {
        let item_uid = self.sol.internal_get_sol_item_uid(item_id)?;
        Ok(ItemStats::new(self.sol, item_uid))
    }
}
