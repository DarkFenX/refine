use rc::ItemCommon;

use crate::{Item, ItemId, SolarSystem};

impl<'r, 's> SolarSystem<'r> {
    #[tracing::instrument(name = "itm-get", level = "trace", skip_all)]
    pub async fn get_item(&'s mut self, item_id: ItemId) -> Result<Item<'r, 's>, ItemGetError> {
        let item_id =
            self.exec_inplace(move |core_sol| core_sol.get_item(&item_id).map(|core_item| core_item.get_item_id()))?;
        let item = Item::new(self, item_id);
        Ok(item)
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct ItemGetError(#[from] pub rc::err::GetItemError);
