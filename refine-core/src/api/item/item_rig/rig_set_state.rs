use crate::{
    Rig,
    sol::{SolCore, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Rig<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_rig_state_inner::<true>(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_set_rig_state(&mut self, rig_uid: UItemId, state: bool) {
        self.internal_set_rig_state_inner::<false>(rig_uid, state);
    }
    fn internal_set_rig_state_inner<const SCREEN: bool>(&mut self, rig_uid: UItemId, state: bool) {
        let u_rig = self.u_data.items.get_mut(rig_uid).dc_rig_mut().unwrap();
        if SCREEN && state == u_rig.get_rig_state() {
            return;
        }
        if self.undo.is_recording() {
            self.undo.record(UndoStep::RigState(rig_uid, u_rig.get_rig_state()));
        }
        let old_a_state = u_rig.get_state();
        u_rig.set_rig_state(state);
        let new_a_state = u_rig.get_state();
        u_rig.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        SolCore::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            rig_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
