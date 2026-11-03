use crate::{
    api::Character,
    sol::{SolarSystem, UndoStep},
    ud::UItemId,
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_character_state(&mut self, character_uid: UItemId, state: bool) {
        let u_character = self.u_data.items.get_mut(character_uid).dc_character_mut().unwrap();
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
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            character_uid,
            old_a_state,
            new_a_state,
            &self.reuse.eupdates,
        );
    }
}

impl<'s> Character<'s> {
    pub fn set_state(&mut self, state: bool) {
        let u_character = self.sol.u_data.items.get(self.uid).dc_character().unwrap();
        if state == u_character.get_character_state() {
            return;
        }
        self.sol.internal_set_character_state(self.uid, state)
    }
}
