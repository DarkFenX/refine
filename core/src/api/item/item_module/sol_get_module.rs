use crate::{
    api::{Module, ModuleMut},
    def::ItemId,
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
};

impl SolarSystem {
    pub fn get_module(&self, item_id: &ItemId) -> Result<Module<'_>, GetModuleError> {
        let module_key = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(module_key).dc_module()?;
        Ok(Module::new(self, module_key))
    }
    pub fn get_module_mut(&mut self, item_id: &ItemId) -> Result<ModuleMut<'_>, GetModuleError> {
        let module_key = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(module_key).dc_module()?;
        Ok(ModuleMut::new(self, module_key))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetModuleError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotModule(#[from] ItemKindMatchError),
}
