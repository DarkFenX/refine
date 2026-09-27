use crate::{api::CharacterMut, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api::item) fn internal_remove_character(&mut self, character_uid: UItemId) {
        SolarSystem::util_remove_character(&mut self.u_data, &mut self.svc, character_uid, &mut self.cache.eupdates);
        let u_character = self.u_data.items.get(character_uid).dc_character().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_character.get_fit_uid());
        u_fit.character = None;
        self.u_data.items.remove(character_uid);
    }
}

impl<'s> CharacterMut<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_character(self.uid);
    }
}
