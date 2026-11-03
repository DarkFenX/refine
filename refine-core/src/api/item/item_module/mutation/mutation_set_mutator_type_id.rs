use crate::{
    SolarSystem,
    ad::AItemId,
    sol::UndoStep,
    ud::{
        UItemId,
        err::{ItemMutatedError, MutatorInplaceError},
    },
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_module_mutator_aid(
        &mut self,
        module_uid: UItemId,
        mutator_type_aid: AItemId,
    ) -> Result<(), ItemMutatedError> {
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        let Some(old_mutator_type_aid) = u_module.get_mutator_type_aid() else {
            return Err(ItemMutatedError);
        };
        if old_mutator_type_aid == mutator_type_aid {
            return Ok(());
        }
        // Item type normally stays the same (mutators share mutated item type, or mutation stays
        // dormant), in which case only attribute values can change
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        match u_module.try_set_mutator_type_aid_inplace(mutator_type_aid, &self.u_data.r_data) {
            Ok(changed_attr_rids) => {
                if self.undo.is_recording() {
                    self.undo
                        .record(UndoStep::ModuleMutator(module_uid, old_mutator_type_aid));
                }
                for attr_rid in changed_attr_rids.into_iter() {
                    self.svc
                        .notify_base_attr_value_changed(&self.u_data, module_uid, attr_rid);
                }
                return Ok(());
            }
            Err(MutatorInplaceError::NotMutated(_)) => unreachable!("item is checked to be mutated"),
            Err(MutatorInplaceError::MutatedTypeChange) => (),
        }
        SolarSystem::util_remove_module_with_charge_act(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            &mut self.reuse.eupdates,
        );
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module
            .set_mutator_type_aid_replace(mutator_type_aid, &self.u_data.r_data)
            .unwrap();
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::ModuleMutator(module_uid, old_mutator_type_aid));
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
