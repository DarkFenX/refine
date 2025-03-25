use crate::{
    err::basic::ItemFoundError,
    sol::{ItemId, SolarSystem, info::ItemInfo},
};

impl SolarSystem {
    pub fn get_item(&self, item_id: &ItemId) -> Result<ItemInfo, GetItemError> {
        let item = self.uad.items.get_item(item_id)?;
        Ok(ItemInfo::from_sol_item(item, self))
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
