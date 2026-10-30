use crate::{
    ad::AItemId,
    api::{CharacterMut, FitMut, ItemTypeId},
    sol::{SolarSystem, UndoStep},
    ud::{UCharacter, UFitId, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fit_character(&mut self, fit_uid: UFitId, type_aid: AItemId) -> UItemId {
        let u_fit = self.u_data.fits.get(fit_uid);
        // Remove old character, if it was set
        if let Some(old_character_uid) = u_fit.character {
            self.internal_remove_character(old_character_uid);
        }
        // Add new character
        let item_id = self.u_data.items.alloc_id();
        let u_character = UCharacter::new(item_id, type_aid, fit_uid, true, &self.u_data.r_data);
        let u_item = UItem::Character(u_character);
        let character_uid = self.u_data.items.add(u_item);
        self.attach_character(fit_uid, character_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::CharacterAdded(character_uid));
        }
        character_uid
    }
    pub(in crate::api) fn internal_reattach_character(&mut self, character_uid: UItemId) {
        self.u_data.items.reattach(character_uid);
        let u_character = self.u_data.items.get(character_uid).dc_character().unwrap();
        let fit_uid = u_character.get_fit_uid();
        self.attach_character(fit_uid, character_uid);
    }
    fn attach_character(&mut self, fit_uid: UFitId, character_uid: UItemId) {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.character = Some(character_uid);
        SolarSystem::util_add_character(&mut self.u_data, &mut self.svc, character_uid, &mut self.cache.eupdates);
    }
}

impl<'s> FitMut<'s> {
    pub fn set_character(&mut self, type_id: ItemTypeId) -> CharacterMut<'_> {
        let character_uid = self.sol.internal_set_fit_character(self.uid, type_id.into_aid());
        CharacterMut::new(self.sol, character_uid)
    }
}
