use crate::{
    sol::{CoreInner, UndoStep},
    ud::{ItemMutationData, UItemId, err::ItemMutatedError},
};

impl CoreInner {
    pub(in crate::api) fn internal_remove_module_mutation(
        &mut self,
        module_uid: UItemId,
    ) -> Result<(), ItemMutatedError> {
        // Checked before services are touched, so that failed requests do nothing
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        if u_module.get_mutator_type_aid().is_none() {
            return Err(ItemMutatedError);
        }
        CoreInner::util_remove_module_with_charge_act(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            &mut self.reuse.eupdates,
        );
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        let item_mutation_data = u_module.unmutate(&self.u_data.r_data).unwrap();
        if self.undo.is_recording() {
            self.undo
                .record_mutation(UndoStep::ModuleMutationRemoved(module_uid), item_mutation_data);
        }
        CoreInner::util_add_module_with_charge_act(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            &mut self.reuse.eupdates,
        );
        Ok(())
    }
    pub(in crate::api) fn internal_restore_module_mutation(
        &mut self,
        module_uid: UItemId,
        item_mutation_data: ItemMutationData,
    ) {
        CoreInner::util_remove_module_with_charge_act(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            &mut self.reuse.eupdates,
        );
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.restore_mutation(item_mutation_data, &self.u_data.r_data);
        CoreInner::util_add_module_with_charge_act(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            &mut self.reuse.eupdates,
        );
    }
}
