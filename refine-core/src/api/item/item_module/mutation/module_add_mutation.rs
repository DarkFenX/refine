use crate::{
    ItemTypeId, Module, Mutation, SolarSystem,
    err::{MutationAddError, basic::ItemNotMutatedError},
    sol::UndoStep,
    ud::{UItemId, UItemMutationRequest},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Module<'s> {
    pub fn mutate(&mut self, mutator_type_id: ItemTypeId) -> Result<Mutation<'_>, MutationAddError> {
        let mutation = UItemMutationRequest {
            mutator_type_aid: mutator_type_id.into_aid(),
            attrs: Vec::new(),
        };
        self.sol.internal_add_module_mutation(self.uid, mutation)?;
        Ok(self.get_mutation().unwrap())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub(in crate::api) fn internal_add_module_mutation(
        &mut self,
        module_uid: UItemId,
        mutation: UItemMutationRequest,
    ) -> Result<(), ItemNotMutatedError> {
        // Checked before services are touched, so that failed requests do nothing
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        if u_module.get_mutator_type_aid().is_some() {
            return Err(ItemNotMutatedError {
                item_id: u_module.get_item_id(),
            });
        }
        SolarSystem::util_remove_module_with_charge_act(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            &mut self.reuse.eupdates,
        );
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.mutate(mutation, &self.u_data.r_data).unwrap();
        if self.undo.is_recording() {
            self.undo.record(UndoStep::ModuleMutationAdded(module_uid));
        }
        SolarSystem::util_add_module_with_charge_act(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            &mut self.reuse.eupdates,
        );
        Ok(())
    }
}
