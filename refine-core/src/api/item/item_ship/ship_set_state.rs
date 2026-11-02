use crate::{
    api::ShipMut,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_ship_state(&mut self, ship_uid: UItemId, state: bool) {
        let u_ship = self.u_data.items.get_mut(ship_uid).dc_ship_mut().unwrap();
        let old_state = u_ship.get_ship_state();
        if state == old_state {
            return;
        }
        if self.undo.is_recording() {
            self.undo.record(UndoStep::ShipState(ship_uid, old_state));
        }
        let old_a_state = u_ship.get_state();
        u_ship.set_ship_state(state);
        let new_a_state = u_ship.get_state();
        u_ship.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            ship_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}

impl<'s> ShipMut<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_ship_state(self.uid, state)
    }
}
