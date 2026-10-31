use crate::{
    sol::{SolarSystem, UndoStep},
    ud::{UAttrMutationRequest, UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_module_mutation(
        &mut self,
        module_uid: UItemId,
    ) -> Result<(), ItemMutatedError> {
        // Checked before services are touched, so that failed requests do nothing
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        if u_module.get_mutator_type_aid().is_none() {
            return Err(ItemMutatedError);
        }
        SolarSystem::util_remove_module_with_charge_act(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            &mut self.cache.eupdates,
        );
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        let item_mutation_data = u_module.unmutate(&self.u_data.r_data).unwrap();
        if self.undo.is_recording() {
            let attr_mutations = item_mutation_data
                .get_attr_rolls()
                .iter()
                .map(|(&attr_aid, &roll)| UAttrMutationRequest {
                    attr_aid,
                    roll: Some(roll),
                })
                .collect();
            self.undo.record_mutation_attrs(
                UndoStep::ModuleMutationRemoved(module_uid, item_mutation_data.get_mutator_type_aid()),
                attr_mutations,
            );
        }
        SolarSystem::util_add_module_with_charge_act(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            &mut self.cache.eupdates,
        );
        Ok(())
    }
}
