use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{
        SolarSystem,
        api::{CharacterMut, FitMut},
    },
    ud::{UCharacter, UEffectUpdates, UFitKey, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_character(
        &mut self,
        fit_key: UFitKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemKey {
        let u_fit = self.u_data.fits.get(fit_key);
        // Remove old character, if it was set
        if let Some(old_item_key) = u_fit.character {
            self.internal_remove_character(old_item_key, reuse_eupdates);
        }
        // Add new character
        let item_id = self.u_data.items.alloc_id();
        let u_character = UCharacter::new(item_id, type_id, fit_key, true, &self.u_data.src);
        let u_item = UItem::Character(u_character);
        let item_key = self.u_data.items.add(u_item);
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.character = Some(item_key);
        SolarSystem::util_add_character(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn set_character(&mut self, type_id: ItemTypeId) -> CharacterMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_key = self
            .sol
            .internal_set_fit_character(self.key, type_id, &mut reuse_eupdates);
        CharacterMut::new(self.sol, item_key)
    }
}
