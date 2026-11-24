use rc::ItemCommon;

use crate::{Item, ItemId, Sol};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'refine, 'sol> Sol<'refine> {
    #[tracing::instrument(name = "itm-get", level = "error", skip_all)]
    pub async fn get_item(&'sol mut self, item_id: ItemId) -> Result<Item<'refine, 'sol>, ItemGetError> {
        let item_id =
            self.exec_inplace(|core_sol| core_sol.get_item(&item_id).map(|core_item| core_item.get_item_id()))?;
        let item = Item::new(self, item_id);
        Ok(item)
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ItemGetError(#[from] pub rc::err::ItemGetError);
