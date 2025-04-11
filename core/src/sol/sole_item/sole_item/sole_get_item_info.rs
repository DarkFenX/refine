use crate::{
    err::basic::ItemFoundError,
    sol::{ItemId, ItemKey, SolarSystem, info::ItemInfo},
};

impl SolarSystem {
    pub fn get_item_info(&self, item_id: &ItemId) -> Result<ItemInfo, GetItemInfoError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_item_info_internal(item_key))
    }
    pub(in crate::sol) fn get_item_info_internal(&self, item_key: ItemKey) -> ItemInfo {
        let item = self.uad.items.get(item_key);
        ItemInfo::from_item(&self.uad, item)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemInfoError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
}
