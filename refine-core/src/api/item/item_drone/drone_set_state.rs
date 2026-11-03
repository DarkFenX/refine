use crate::{
    Drone, MinionState,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Drone<'s> {
    pub fn set_state(&mut self, state: MinionState) {
        let u_drone = self.sol.u_data.items.get(self.uid).dc_drone().unwrap();
        if state == u_drone.get_drone_state() {
            return;
        }
        self.sol.internal_set_drone_state(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_drone_state(&mut self, drone_uid: UItemId, state: MinionState) {
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::DroneState(drone_uid, u_drone.get_drone_state()));
        }
        let old_a_state = u_drone.get_state();
        u_drone.set_drone_state(state);
        let new_a_state = u_drone.get_state();
        u_drone.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        CoreInner::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            drone_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
