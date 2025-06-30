use crate::{
    def::{ItemKey, ItemTypeId},
    err::basic::ItemNotMutatedError,
    misc::ItemMutationRequest,
    sol::{
        SolarSystem,
        api::{AddMutationError, ModuleMut, MutationMut},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_module_mutation(
        &mut self,
        item_key: ItemKey,
        mutation: ItemMutationRequest,
    ) -> Result<(), ItemNotMutatedError> {
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_remove_module_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        if let Err(error) = uad_module.mutate(&self.uad.src, mutation) {
            let uad_item = self.uad.items.get(item_key);
            SolarSystem::util_add_module_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
            return Err(error);
        }
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_module_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        Ok(())
    }
}

impl<'a> ModuleMut<'a> {
    pub fn mutate(&mut self, mutator_id: ItemTypeId) -> Result<MutationMut<'_>, AddMutationError> {
        let mutation = ItemMutationRequest {
            mutator_id,
            attrs: Vec::new(),
        };
        self.sol.internal_add_module_mutation(self.key, mutation)?;
        Ok(self.get_mutation_mut().unwrap())
    }
}
