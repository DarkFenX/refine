use crate::{
    err::basic::ItemNotMutatedError,
    sol::{ItemKey, SolarSystem, api::ModuleMut, uad::item::ItemAddMutation},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_add_module_mutation(
        &mut self,
        item_key: ItemKey,
        mutation: ItemAddMutation,
    ) -> Result<(), ItemNotMutatedError> {
        let uad_item = self.uad.items.get(item_key);
        self.svc.unload_item(&self.uad, item_key, uad_item);
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        if let Err(error) = uad_module.mutate(&self.uad.src, mutation) {
            let item = self.uad.items.get(item_key);
            self.svc.load_item(&self.uad, item_key, item);
            return Err(error);
        }
        let uad_item = self.uad.items.get(item_key);
        self.svc.load_item(&self.uad, item_key, uad_item);
        Ok(())
    }
}

impl<'a> ModuleMut<'a> {
    pub fn mutate(&mut self, mutation: ItemAddMutation) -> Result<(), AddModuleMutationError> {
        self.sol.internal_add_module_mutation(self.key, mutation)?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddModuleMutationError {
    #[error("{0}")]
    MutationAlreadySet(#[from] ItemNotMutatedError),
}
