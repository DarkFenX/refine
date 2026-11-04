use crate::{
    Charge,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Charge<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.core.internal_set_charge_state_inner::<true>(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_charge_state(&mut self, charge_uid: UItemId, state: bool) {
        self.internal_set_charge_state_inner::<false>(charge_uid, state);
    }
    fn internal_set_charge_state_inner<const SCREEN: bool>(&mut self, charge_uid: UItemId, state: bool) {
        let u_charge = self.u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
        if SCREEN && state == !u_charge.get_force_disabled() {
            return;
        }
        if self.undo.is_recording() {
            self.undo
                .record(UndoStep::ChargeState(charge_uid, !u_charge.get_force_disabled()));
        }
        let old_a_state = u_charge.get_state();
        u_charge.set_force_disabled(!state);
        let new_a_state = u_charge.get_state();
        u_charge.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        CoreInner::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            charge_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
