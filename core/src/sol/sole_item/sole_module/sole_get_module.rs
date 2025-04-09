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

#[derive(thiserror::Error, Debug)]
pub enum GetModuleError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotModule(#[from] ItemKindMatchError),
}
