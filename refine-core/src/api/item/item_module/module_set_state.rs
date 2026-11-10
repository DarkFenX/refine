use crate::{
    Module, ModuleState,
    sol::{SolCore, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Module<'s> {
    pub fn set_state(&mut self, state: ModuleState) {
        self.sol.internal_set_module_state_inner::<true>(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_set_module_state(&mut self, module_uid: UItemId, state: ModuleState) {
        self.internal_set_module_state_inner::<false>(module_uid, state);
    }
    fn internal_set_module_state_inner<const SCREEN: bool>(&mut self, module_uid: UItemId, state: ModuleState) {
        // Update user data for module
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        if SCREEN && state == u_module.get_module_state() {
            return;
        }
        let charge_uid = u_module.get_charge_uid();
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::ModuleState(module_uid, u_module.get_module_state()));
        }
        let old_a_state = u_module.get_state();
        u_module.set_module_state(state);
        let new_a_state = u_module.get_state();
        u_module.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        // Update services for module
        SolCore::util_switch_item_state(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
        if let Some(charge_activated) = self.reuse.eupdates.charge
            && let Some(charge_uid) = charge_uid
        {
            SolCore::util_process_charge_activation(
                &mut self.u_data,
                &mut self.svc,
                charge_uid,
                charge_activated,
                &mut self.reuse.eupdates,
            );
        }
    }
}
