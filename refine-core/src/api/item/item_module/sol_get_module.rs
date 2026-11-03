use crate::{
    ItemId, Module, SolarSystem,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

impl SolarSystem {
    pub fn get_module(&mut self, item_id: &ItemId) -> Result<Module<'_>, ModuleGetError> {
        let module_uid = self.u_data.items.int_id_by_ext_id_err(item_id)?;
        self.u_data.items.get(module_uid).dc_module()?;
        Ok(Module::new(self, module_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ModuleGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotModule(#[from] ItemKindMatchError),
}
