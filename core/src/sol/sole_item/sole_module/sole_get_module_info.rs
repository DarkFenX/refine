use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::ModuleInfo},
};

impl SolarSystem {
    pub fn get_module_info(&self, item_id: &ItemId) -> Result<ModuleInfo, GetModuleInfoError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_module_info_internal(item_key)?)
    }
    pub(in crate::sol) fn get_module_info_internal(&self, item_key: ItemKey) -> Result<ModuleInfo, ItemKindMatchError> {
        let module = self.uad.items.get(item_key).get_module()?;
        Ok(ModuleInfo::from_module(&self.uad, module))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetModuleInfoError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotModule(#[from] ItemKindMatchError),
}
