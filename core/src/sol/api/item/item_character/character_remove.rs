use crate::{
    def::ItemKey,
    sol::{SolarSystem, api::CharacterMut},
    uad::UadEffectUpdates,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_character(
        &mut self,
        item_key: ItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        let uad_item = self.uad.items.get(item_key);
        let uad_character = uad_item.get_character().unwrap();
        SolarSystem::util_remove_character(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        let uad_fit = self.uad.fits.get_mut(uad_character.get_fit_key());
        uad_fit.character = None;
        self.uad.items.remove(item_key);
    }
}

impl<'a> CharacterMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol.internal_remove_character(self.key, &mut reuse_eupdates);
    }
}
