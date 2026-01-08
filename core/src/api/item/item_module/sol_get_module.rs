use crate::{
    api::{Module, ModuleMut},
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::SolarSystem,
    ud::ItemId,
};

impl SolarSystem {
    pub fn get_module(&self, item_id: &ItemId) -> Result<Module<'_>, GetModuleError> {
        let module_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(module_uid).dc_module()?;
        Ok(Module::new(self, module_uid))
    }
    pub fn get_module_mut(&mut self, item_id: &ItemId) -> Result<ModuleMut<'_>, GetModuleError> {
        let module_uid = self.u_data.items.iid_by_xid_err(item_id)?;
        self.u_data.items.get(module_uid).dc_module()?;
        Ok(ModuleMut::new(self, module_uid))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetModuleError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotModule(#[from] ItemKindMatchError),
}
