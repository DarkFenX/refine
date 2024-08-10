use crate::{
    defs::SolItemId,
    err::ItemFoundError,
    sol::{item_info::SolItemInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_item_info(&self, item_id: &SolItemId) -> Result<SolItemInfo, GetItemInfoError> {
        let item = self.items.get_item(item_id)?;
        Ok(SolItemInfo::from_sol_item(item, self))
    }
}

#[derive(Debug)]
pub enum GetItemInfoError {
    ItemNotFound(ItemFoundError),
}
impl From<ItemFoundError> for GetItemInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl std::error::Error for GetItemInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetItemInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
        }
    }
}
