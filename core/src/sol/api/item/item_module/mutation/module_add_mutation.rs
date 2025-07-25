use crate::{
    def::ItemTypeId,
    err::basic::ItemNotMutatedError,
    misc::ItemMutationRequest,
    sol::{
        SolarSystem,
        api::{AddMutationError, ModuleMut, MutationMut},
    },
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_module_mutation(
        &mut self,
        item_key: UItemKey,
        mutation: ItemMutationRequest,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemNotMutatedError> {
        let u_item = self.u_data.items.get(item_key);
        SolarSystem::util_remove_module_with_projs(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        let u_module = self.u_data.items.get_mut(item_key).get_module_mut().unwrap();
        if let Err(error) = u_module.mutate(mutation, reuse_eupdates, &self.u_data.src) {
            let u_item = self.u_data.items.get(item_key);
            // When util remove function was called, module was removed from services, but its
            // running effects container stayed as-is, since the request to mutate failed; to
            // restart all effects, refill the effect updates container with effects which are still
            // marked as running on the module
            u_item.start_all_reffs(reuse_eupdates, &self.u_data.src);
            SolarSystem::util_add_module_with_projs(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
            return Err(error);
        }
        SolarSystem::util_add_module_with_projs(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
        Ok(())
    }
}

impl<'a> ModuleMut<'a> {
    pub fn mutate(&mut self, mutator_id: ItemTypeId) -> Result<MutationMut<'_>, AddMutationError> {
        let mutation = ItemMutationRequest {
            mutator_id,
            attrs: Vec::new(),
        };
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_add_module_mutation(self.key, mutation, &mut reuse_eupdates)?;
        Ok(self.get_mutation_mut().unwrap())
    }
}
