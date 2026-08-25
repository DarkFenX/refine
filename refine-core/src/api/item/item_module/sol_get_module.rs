use crate::{
    api::{Module, ModuleMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_module(&self, item_id: &ItemId) -> Result<Module<'_>, ModuleGetError> {
        let module_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(module_uid).dc_module()?;
        Ok(Module::new(self, module_uid))
    }
    pub fn get_module_mut(&mut self, item_id: &ItemId) -> Result<ModuleMut<'_>, ModuleGetError> {
        let module_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(module_uid).dc_module()?;
        Ok(ModuleMut::new(self, module_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ModuleGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotModule(#[from] ItemKindMatchError),
}
