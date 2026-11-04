use crate::{
    Implant,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Implant<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_implant_state_inner::<true>(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_implant_state(&mut self, implant_uid: UItemId, state: bool) {
        self.internal_set_implant_state_inner::<false>(implant_uid, state);
    }
    fn internal_set_implant_state_inner<const SCREEN: bool>(&mut self, implant_uid: UItemId, state: bool) {
        let u_implant = self.u_data.items.get_mut(implant_uid).dc_implant_mut().unwrap();
        if SCREEN && state == u_implant.get_implant_state() {
            return;
        }
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::ImplantState(implant_uid, u_implant.get_implant_state()));
        }
        let old_a_state = u_implant.get_state();
        u_implant.set_implant_state(state);
        let new_a_state = u_implant.get_state();
        u_implant.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        CoreInner::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            implant_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
