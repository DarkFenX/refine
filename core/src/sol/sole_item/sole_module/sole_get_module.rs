use crate::{
    defs::SolItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolModuleInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_module(&self, item_id: &SolItemId) -> Result<SolModuleInfo, GetModuleError> {
        let module = self.items.get_item(item_id)?.get_module()?;
        Ok(self.make_module_info(module))
    }
}

#[derive(Debug)]
pub enum GetModuleError {
    ItemNotFound(ItemFoundError),
    ItemIsNotModule(ItemKindMatchError),
}
impl std::error::Error for GetModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotModule(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotModule(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetModuleError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetModuleError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotModule(error)
    }
}
