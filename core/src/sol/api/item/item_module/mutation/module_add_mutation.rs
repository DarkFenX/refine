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
        SolarSystem::util_remove_module_with_charge_act(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        let u_module = self.u_data.items.get_mut(item_key).get_module_mut().unwrap();
        if let Err(error) = u_module.mutate(mutation, &self.u_data.src) {
            SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
            return Err(error);
        }
        SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
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
