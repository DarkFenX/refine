use crate::{
    sol::{SolarSystem, UndoStep},
    ud::{UAttrMutationRequest, UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_change_module_mutation_attrs(
        &mut self,
        module_uid: UItemId,
        mut attr_mutations: Vec<UAttrMutationRequest>,
    ) -> Result<(), ItemMutatedError> {
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        let changed_attr_rids = u_module.change_mutation_attrs(&self.u_data.r_data, &mut attr_mutations)?;
        if self.undo.is_recording() {
            // Reversed just to handle multiple requests for the same attribute correctly
            attr_mutations.reverse();
            self.undo
                .record_mutation_attrs(UndoStep::ModuleMutationAttrs(module_uid), attr_mutations);
        }
        for attr_rids in changed_attr_rids {
            self.svc
                .notify_base_attr_value_changed(&self.u_data, module_uid, attr_rids);
        }
        Ok(())
    }
}
