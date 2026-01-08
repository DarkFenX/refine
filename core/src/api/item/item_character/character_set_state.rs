use crate::{
    api::CharacterMut,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_character_state(
        &mut self,
        character_uid: UItemId,
        state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_character = self.u_data.items.get_mut(character_uid).dc_character_mut().unwrap();
        let old_a_state = u_character.get_state();
        u_character.set_character_state(state);
        let new_a_state = u_character.get_state();
        u_character.update_reffs(reuse_eupdates, &self.u_data.src);
        SolarSystem::util_switch_item_state(
            &self.u_data,
            &mut self.svc,
            character_uid,
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
            .internal_set_character_state(self.uid, state, &mut reuse_eupdates)
    }
}
