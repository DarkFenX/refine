use crate::{
    api::ChargeMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_charge_state(&mut self, charge_uid: UItemId, state: bool) {
        let u_charge = self.u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
        let old_state = !u_charge.get_force_disabled();
        if state == old_state {
            return;
        }
        if self.undo.is_recording() {
            self.undo.record(UndoStep::ChargeState(charge_uid, old_state));
        }
        let old_a_state = u_charge.get_state();
        u_charge.set_force_disabled(!state);
        let new_a_state = u_charge.get_state();
        u_charge.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            charge_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}

impl<'s> ChargeMut<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_charge_state(self.uid, state);
    }
}
