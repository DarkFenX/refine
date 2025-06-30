use crate::{
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{
        SolarSystem,
        api::{Module, ModuleMut},
    },
};

impl SolarSystem {
    pub fn get_module(&self, item_id: &ItemId) -> Result<Module<'_>, GetModuleError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_module()?;
        Ok(Module::new(self, item_key))
    }
    pub fn get_module_mut(&mut self, item_id: &ItemId) -> Result<ModuleMut<'_>, GetModuleError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.uad.items.get(item_key).get_module()?;
        Ok(ModuleMut::new(self, item_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetModuleError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotModule(#[from] ItemKindMatchError),
}
