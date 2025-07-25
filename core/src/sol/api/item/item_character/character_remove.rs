use crate::{
    sol::{SolarSystem, api::CharacterMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_character(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(item_key);
        let u_character = u_item.get_character().unwrap();
        SolarSystem::util_remove_character(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        let u_fit = self.u_data.fits.get_mut(u_character.get_fit_key());
        u_fit.character = None;
        self.u_data.items.remove(item_key);
    }
}

impl<'a> CharacterMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_character(self.key, &mut reuse_eupdates);
    }
}
