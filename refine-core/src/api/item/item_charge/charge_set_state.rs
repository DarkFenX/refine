use crate::{
    api::{ChargeMut, UndoStep},
    sol::SolarSystem,
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_charge_state(&mut self, charge_uid: UItemId, state: bool) -> UndoStep {
        let u_charge = self.u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
        let undo_step = UndoStep::ChargeState(u_charge.get_item_id(), !u_charge.get_force_disabled());
        let old_a_state = u_charge.get_state();
        u_charge.set_force_disabled(!state);
        let new_a_state = u_charge.get_state();
        u_charge.update_reffs(&mut self.cache.eupdates, &self.u_data.r_data);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            charge_uid,
            old_a_state,
            new_a_state,
            &self.cache.eupdates,
        );
        undo_step
    }
}

impl<'s> ChargeMut<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_charge_state(self.uid, state);
    }
}
