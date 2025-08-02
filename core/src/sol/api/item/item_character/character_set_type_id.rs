use crate::{
    ad::AItemId,
    def::ItemTypeId,
    sol::{SolarSystem, api::CharacterMut},
    ud::{UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_character_type_id(
        &mut self,
        item_key: UItemKey,
        type_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = self.u_data.items.get(item_key);
        if u_item.get_type_id() == type_id {
            return;
        }
        SolarSystem::util_remove_character(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        let u_character = self.u_data.items.get_mut(item_key).get_character_mut().unwrap();
        u_character.set_type_id(type_id, &self.u_data.src);
        SolarSystem::util_add_character(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
    }
}

impl<'a> CharacterMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_set_character_type_id(self.key, type_id, &mut reuse_eupdates)
    }
}
