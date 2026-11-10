use crate::{
    Stance,
    sol::{SolCore, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Stance<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_stance_state_inner::<true>(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_set_stance_state(&mut self, stance_uid: UItemId, state: bool) {
        self.internal_set_stance_state_inner::<false>(stance_uid, state);
    }
    fn internal_set_stance_state_inner<const SCREEN: bool>(&mut self, stance_uid: UItemId, state: bool) {
        let u_stance = self.u_data.items.get_mut(stance_uid).dc_stance_mut().unwrap();
        if SCREEN && state == u_stance.get_stance_state() {
            return;
        }
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::StanceState(stance_uid, u_stance.get_stance_state()));
        }
        let old_a_state = u_stance.get_state();
        u_stance.set_stance_state(state);
        let new_a_state = u_stance.get_state();
        u_stance.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        SolCore::util_switch_item_state(
            &mut self.u_data,
            &mut self.svc,
            stance_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
