use crate::{
    api::{AddMutationError, ItemTypeId, ModuleMut, MutationMut},
    err::basic::ItemNotMutatedError,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId, UItemMutationRequest},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_module_mutation(
        &mut self,
        module_uid: UItemId,
        mutation: UItemMutationRequest,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemNotMutatedError> {
        SolarSystem::util_remove_module_with_charge_act(&mut self.u_data, &mut self.svc, module_uid, reuse_eupdates);
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        if let Err(error) = u_module.mutate(mutation, &self.u_data.src) {
            SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, module_uid, reuse_eupdates);
            return Err(error);
        }
        SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, module_uid, reuse_eupdates);
        Ok(())
    }
}

impl<'a> ModuleMut<'a> {
    pub fn mutate(&mut self, mutator_type_id: ItemTypeId) -> Result<MutationMut<'_>, AddMutationError> {
        let mutation = UItemMutationRequest {
            mutator_type_aid: mutator_type_id.into_aid(),
            attrs: Vec::new(),
        };
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_add_module_mutation(self.uid, mutation, &mut reuse_eupdates)?;
        Ok(self.get_mutation_mut().unwrap())
    }
}
