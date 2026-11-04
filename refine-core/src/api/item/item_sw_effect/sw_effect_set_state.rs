use crate::{
    SwEffect,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> SwEffect<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.core.internal_set_sw_effect_state_inner::<true>(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_sw_effect_state(&mut self, sw_effect_uid: UItemId, state: bool) {
        self.internal_set_sw_effect_state_inner::<false>(sw_effect_uid, state);
    }
    fn internal_set_sw_effect_state_inner<const SCREEN: bool>(&mut self, sw_effect_uid: UItemId, state: bool) {
        let u_sw_effect = self.u_data.items.get_mut(sw_effect_uid).dc_sw_effect_mut().unwrap();
        if SCREEN && state == u_sw_effect.get_sw_effect_state() {
            return;
        }
        if self.undo.is_recording() {
            self.undo.record(UndoStep::SwEffectState(
                sw_effect_uid,
                u_sw_effect.get_sw_effect_state(),
            ));
        }
        let old_a_state = u_sw_effect.get_state();
        u_sw_effect.set_sw_effect_state(state);
        let new_a_state = u_sw_effect.get_state();
        u_sw_effect.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        CoreInner::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            sw_effect_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
