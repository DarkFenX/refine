use crate::{
    ad::AItemId,
    api::{CharacterMut, ItemTypeId},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_character_type_id(
        &mut self,
        character_uid: UItemId,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(character_uid);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_character(&mut self.u_data, &mut self.svc, character_uid, reuse_eupdates);
        let u_character = self.u_data.items.get_mut(character_uid).dc_character_mut().unwrap();
        u_character.set_type_id(type_id, &self.u_data.src);
        SolarSystem::util_add_character(&mut self.u_data, &mut self.svc, character_uid, reuse_eupdates);
    }
}

impl<'a> CharacterMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_character_type_id(self.uid, type_id.into_aid(), &mut reuse_eupdates)
    }
}
