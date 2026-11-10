use crate::{
    FwEffect,
    sol::{SolCore, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FwEffect<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_fw_effect_state_inner::<true>(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_set_fw_effect_state(&mut self, fw_effect_uid: UItemId, state: bool) {
        self.internal_set_fw_effect_state_inner::<false>(fw_effect_uid, state);
    }
    fn internal_set_fw_effect_state_inner<const SCREEN: bool>(&mut self, fw_effect_uid: UItemId, state: bool) {
        let u_fw_effect = self.u_data.items.get_mut(fw_effect_uid).dc_fw_effect_mut().unwrap();
        if SCREEN && state == u_fw_effect.get_fw_effect_state() {
            return;
        }
        if self.undo.is_recording() {
            self.undo.record(UndoStep::FwEffectState(
                fw_effect_uid,
                u_fw_effect.get_fw_effect_state(),
            ));
        }
        let old_a_state = u_fw_effect.get_state();
        u_fw_effect.set_fw_effect_state(state);
        let new_a_state = u_fw_effect.get_state();
        u_fw_effect.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        SolCore::util_switch_item_state(
            &mut self.u_data,
            &mut self.svc,
            fw_effect_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
