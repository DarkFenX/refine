use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError, ItemNotMutatedError},
    sol::{ItemId, ItemKey, SolarSystem, uad::item::ItemAddMutation},
};

impl SolarSystem {
    pub fn add_module_mutation(
        &mut self,
        item_id: &ItemId,
        mutation: ItemAddMutation,
    ) -> Result<(), AddModuleMutationError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        self.add_module_mutation_internal(item_key, mutation)
    }
    pub(in crate::sol) fn add_module_mutation_internal(
        &mut self,
        item_key: ItemKey,
        mutation: ItemAddMutation,
    ) -> Result<(), AddModuleMutationError> {
        let item = self.uad.items.get(item_key);
        self.svc.unload_item(&self.uad, item_key, item);
        let module = match self.uad.items.get_mut(item_key).get_module_mut() {
            Ok(module) => module,
            Err(error) => {
                let item = self.uad.items.get(item_key);
                self.svc.load_item(&self.uad, item_key, item);
                return Err(error.into());
            }
        };
        if let Err(error) = module.mutate(&self.uad.src, mutation) {
            let item = self.uad.items.get(item_key);
            self.svc.load_item(&self.uad, item_key, item);
            return Err(error.into());
        }
        let item = self.uad.items.get(item_key);
        self.svc.load_item(&self.uad, item_key, item);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddModuleMutationError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotModule(#[from] ItemKindMatchError),
    #[error("{0}")]
    MutationAlreadySet(#[from] ItemNotMutatedError),
}
