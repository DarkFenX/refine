use crate::{
    ad::AItemId,
    api::{CharacterMut, FitMut, ItemTypeId},
    sol::SolarSystem,
    ud::{UCharacter, UEffectUpdates, UFitId, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fit_character(
        &mut self,
        fit_uid: UFitId,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get(fit_uid);
        // Remove old character, if it was set
        if let Some(old_character_uid) = u_fit.character {
            self.internal_remove_character(old_character_uid, reuse_eupdates);
        }
        // Add new character
        let item_id = self.u_data.items.alloc_id();
        let u_character = UCharacter::new(item_id, type_id, fit_uid, true, &self.u_data.src);
        let u_item = UItem::Character(u_character);
        let character_uid = self.u_data.items.add(u_item);
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.character = Some(character_uid);
        SolarSystem::util_add_character(&mut self.u_data, &mut self.svc, character_uid, reuse_eupdates);
        character_uid
    }
}

impl<'a> FitMut<'a> {
    pub fn set_character(&mut self, type_id: ItemTypeId) -> CharacterMut<'_> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let character_uid = self
            .sol
            .internal_set_fit_character(self.uid, type_id.into_aid(), &mut reuse_eupdates);
        CharacterMut::new(self.sol, character_uid)
    }
}
