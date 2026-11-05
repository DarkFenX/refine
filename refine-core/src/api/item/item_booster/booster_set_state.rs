use crate::{
    Booster,
    sol::{SolCore, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Booster<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_booster_state_inner::<true>(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_set_booster_state(&mut self, booster_uid: UItemId, state: bool) {
        self.internal_set_booster_state_inner::<false>(booster_uid, state);
    }
    fn internal_set_booster_state_inner<const SCREEN: bool>(&mut self, booster_uid: UItemId, state: bool) {
        let u_booster = self.u_data.items.get_mut(booster_uid).dc_booster_mut().unwrap();
        if SCREEN && state == u_booster.get_booster_state() {
            return;
        }
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::BoosterState(booster_uid, u_booster.get_booster_state()));
        }
        let old_a_state = u_booster.get_state();
        u_booster.set_booster_state(state);
        let new_a_state = u_booster.get_state();
        u_booster.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        SolCore::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            booster_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
