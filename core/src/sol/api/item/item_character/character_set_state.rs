use crate::{
    sol::{SolarSystem, api::CharacterMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_character_state(
        &mut self,
        item_key: UItemKey,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_character = self.u_data.items.get_mut(item_key).get_character_mut().unwrap();
        let old_a_state = u_character.get_state();
        u_character.set_character_state(state);
        let new_a_state = u_character.get_state();
        u_character.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            item_key,
            old_a_state,
            new_a_state,
            reuse_eupdates,
        );
    }
}

impl<'a> CharacterMut<'a> {
    pub fn set_state(&mut self, state: bool) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_character_state(self.key, state, &mut reuse_eupdates)
    }
}
