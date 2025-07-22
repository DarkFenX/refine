use crate::{
    def::ItemTypeId,
    err::basic::ItemNotMutatedError,
    misc::ItemMutationRequest,
    sol::{
        SolarSystem,
        api::{AddMutationError, ModuleMut, MutationMut},
    },
    uad::{UadEffectUpdates, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_module_mutation(
        &mut self,
        item_key: UadItemKey,
        mutation: ItemMutationRequest,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> Result<(), ItemNotMutatedError> {
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_remove_module_with_projs(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        if let Err(error) = uad_module.mutate(mutation, reuse_eupdates, &self.uad.src) {
            let uad_item = self.uad.items.get(item_key);
            // When util remove function was called, module was removed from services, but its
            // running effects container stayed as-is, since the request to mutate failed; to
            // restart all effects, refill the effect updates container with effects which are still
            // marked as running on the module
            uad_item.start_all_reffs(reuse_eupdates, &self.uad.src);
            SolarSystem::util_add_module_with_projs(&self.uad, &mut self.svc, item_key, reuse_eupdates);
            return Err(error);
        }
        SolarSystem::util_add_module_with_projs(&self.uad, &mut self.svc, item_key, reuse_eupdates);
        Ok(())
    }
}

impl<'a> ModuleMut<'a> {
    pub fn mutate(&mut self, mutator_id: ItemTypeId) -> Result<MutationMut<'_>, AddMutationError> {
        let mutation = ItemMutationRequest {
            mutator_id,
            attrs: Vec::new(),
        };
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol
            .internal_add_module_mutation(self.key, mutation, &mut reuse_eupdates)?;
        Ok(self.get_mutation_mut().unwrap())
    }
}
