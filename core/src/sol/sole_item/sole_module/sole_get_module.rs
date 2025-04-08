use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::ModuleInfo},
};

impl SolarSystem {
    pub fn get_module(&self, item_id: &ItemId) -> Result<ModuleInfo, GetModuleError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_module_internal(item_key)?)
    }
    pub(in crate::sol) fn get_module_internal(&self, item_key: ItemKey) -> Result<ModuleInfo, ItemKindMatchError> {
        let module = self.uad.items.get(item_key).get_module()?;
        Ok(ModuleInfo::from_module(&self.uad, module))
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
