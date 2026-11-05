use crate::{
    Character,
    sol::{SolCore, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Character<'s> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_character_state_inner::<true>(self.uid, state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_set_character_state(&mut self, character_uid: UItemId, state: bool) {
        self.internal_set_character_state_inner::<false>(character_uid, state);
    }
    fn internal_set_character_state_inner<const SCREEN: bool>(&mut self, character_uid: UItemId, state: bool) {
        let u_character = self.u_data.items.get_mut(character_uid).dc_character_mut().unwrap();
        if SCREEN && state == u_character.get_character_state() {
            return;
        }
        if self.undo.is_recording() {
            self.undo.record(UndoStep::CharacterState(
                character_uid,
                u_character.get_character_state(),
            ));
        }
        let old_a_state = u_character.get_state();
        u_character.set_character_state(state);
        let new_a_state = u_character.get_state();
        u_character.update_reffs(&mut self.reuse.eupdates, &self.u_data.r_data);
        SolCore::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            character_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}
