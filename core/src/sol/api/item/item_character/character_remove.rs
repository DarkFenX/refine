use crate::sol::{ItemKey, SolarSystem, api::CharacterMut};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_character(&mut self, item_key: ItemKey) {
        let uad_item = self.uad.items.get(item_key);
        let uad_character = uad_item.get_character().unwrap();
        self.svc.remove_item(&self.uad, item_key, uad_item);
        let uad_fit = self.uad.fits.get_mut(uad_character.get_fit_key());
        uad_fit.character = None;
        self.uad.items.remove(item_key);
    }
}

impl<'a> CharacterMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_character(self.key);
    }
}
