use crate::{Autocharge, SolarSystem, sol::UndoStep, ud::UItemId};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Autocharge<'s> {
    pub fn set_state(&mut self, state: bool) {
        let u_autocharge = self.sol.u_data.items.get(self.uid).dc_autocharge().unwrap();
        if state == !u_autocharge.get_force_disabled() {
            return;
        }
        self.sol.internal_set_autocharge_state(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub(in crate::api) fn internal_set_autocharge_state(&mut self, autocharge_uid: UItemId, state: bool) {
        let u_autocharge = self.u_data.items.get_mut(autocharge_uid).dc_autocharge_mut().unwrap();
        if self.undo.is_recording() {
            self.undo.record(UndoStep::AutochargeState(
                autocharge_uid,
                !u_autocharge.get_force_disabled(),
            ));
        }
        let old_r_state = u_autocharge.get_state();
        u_autocharge.set_force_disabled(!state);
        let new_r_state = u_autocharge.get_state();
        u_autocharge.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            autocharge_uid,
            old_r_state,
            new_r_state,
            &self.reuse.eupdates,
        );
    }
}
