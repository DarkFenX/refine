use crate::{
    ad,
    def::{FitKey, ItemKey, ItemTypeId},
    sol::{
        SolarSystem,
        api::{CharacterMut, FitMut},
    },
    uad::{UadCharacter, UadEffectUpdates, UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_character(
        &mut self,
        fit_key: FitKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> ItemKey {
        let uad_fit = self.uad.fits.get(fit_key);
        // Remove old character, if it was set
        if let Some(old_item_key) = uad_fit.character {
            self.internal_remove_character(old_item_key, reuse_eupdates);
        }
        // Add new character
        let item_id = self.uad.items.alloc_id();
        let uad_character = UadCharacter::new(item_id, a_item_id, fit_key, true, &self.uad.src, reuse_eupdates);
        let uad_item = UadItem::Character(uad_character);
        let item_key = self.uad.items.add(uad_item);
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.character = Some(item_key);
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_character(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn set_character(&mut self, type_id: ItemTypeId) -> CharacterMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self
            .sol
            .internal_set_fit_character(self.key, type_id, &mut reuse_eupdates);
        CharacterMut::new(self.sol, item_key)
    }
}
