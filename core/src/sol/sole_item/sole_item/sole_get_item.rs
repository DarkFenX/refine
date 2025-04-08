use crate::{
    err::basic::ItemFoundError,
    sol::{ItemId, ItemKey, SolarSystem, info::ItemInfo},
};

impl SolarSystem {
    pub fn get_item(&self, item_id: &ItemId) -> Result<ItemInfo, GetItemError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_item_internal(item_key))
    }
    pub(in crate::sol) fn get_item_internal(&self, item_key: ItemKey) -> ItemInfo {
        let item = self.uad.items.get(item_key);
        ItemInfo::from_item(&self.uad, item)
    }
}

#[derive(Debug)]
pub enum GetItemError {
    ItemNotFound(ItemFoundError),
}
impl std::error::Error for GetItemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetItemError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
