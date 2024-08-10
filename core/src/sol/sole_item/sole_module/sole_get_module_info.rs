use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolModuleInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_module_info(&self, item_id: &SolItemId) -> Result<SolModuleInfo, GetModuleInfoError> {
        let module = self.items.get_item(item_id)?.get_module()?;
        Ok(self.make_module_info(module))
    }
}

#[derive(Debug)]
pub enum GetModuleInfoError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
}
impl From<ItemFoundError> for GetModuleInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetModuleInfoError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotModule(error)
    }
}
impl std::error::Error for GetModuleInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotModule(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetModuleInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotModule(e) => e.fmt(f),
        }
    }
}
