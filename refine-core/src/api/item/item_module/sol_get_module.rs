use crate::{
    ItemId, Module, Sol,
    err::{ItemKindMatchError, basic::ItemFoundError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn get_module(&mut self, item_id: &ItemId) -> Result<Module<'_>, ModuleGetError> {
        let sol = self.inner_mut();
        let module_uid = sol.u_data.items.int_id_by_ext_id_err(item_id)?;
        sol.u_data.items.get(module_uid).dc_module()?;
        Ok(Module::new(sol, module_uid))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ModuleGetError {
    #[error(transparent)]
    ItemNotFound(#[from] ItemFoundError),
    #[error(transparent)]
    ItemIsNotModule(#[from] ItemKindMatchError),
}
