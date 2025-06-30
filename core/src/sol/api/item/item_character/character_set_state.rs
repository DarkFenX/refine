use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::CharacterMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_character_state(&mut self, item_key: ItemKey, state: bool) {
        let uad_character = self.uad.items.get_mut(item_key).get_character_mut().unwrap();
        let old_a_state = uad_character.get_a_state();
        uad_character.set_character_state(state);
        let new_a_state = uad_character.get_a_state();
        SolarSystem::util_switch_item_state(
            &self.uad,
            &mut self.svc,
            &mut self.reffs,
            item_key,
            old_a_state,
            new_a_state,
        );
    }
}

impl<'a> CharacterMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        self.sol.internal_set_character_state(self.key, state)
    }
}
