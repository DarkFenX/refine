use crate::{
    err::basic::ItemMutatedError,
    sol::{ItemKey, SolarSystem, api::ModuleMut},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_module_mutation(
        &mut self,
        item_key: ItemKey,
    ) -> Result<(), ItemMutatedError> {
        let uad_item = self.uad.items.get(item_key);
        let uad_module = uad_item.get_module().unwrap();
        if !uad_module.has_mutation_data() {
            return Err(ItemMutatedError {
                item_id: uad_module.get_item_id(),
            });
        }
        self.svc.unload_item(&self.uad, item_key, uad_item);
        self.uad
            .items
            .get_mut(item_key)
            .get_module_mut()
            .unwrap()
            .unmutate(&self.uad.src)
            .unwrap();
        let uad_item = self.uad.items.get(item_key);
        self.svc.load_item(&self.uad, item_key, uad_item);
        Ok(())
    }
}

impl<'a> ModuleMut<'a> {
    pub fn unmutate(self) -> Result<Self, RemoveModuleMutationError> {
        self.sol.internal_remove_module_mutation(self.key)?;
        Ok(self)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveModuleMutationError {
    #[error("{0}")]
    MutationNotSet(#[from] ItemMutatedError),
}
