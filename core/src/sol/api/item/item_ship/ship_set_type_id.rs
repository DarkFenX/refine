use crate::{
    ad,
    sol::{ItemKey, ItemTypeId, SolarSystem, api::ShipMut},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_ship_a_item_id(&mut self, item_key: ItemKey, a_item_id: ad::AItemId) {
        let uad_item = self.uad.items.get(item_key);
        if uad_item.get_a_item_id() == a_item_id {
            return;
        }
        SolarSystem::unload_ship(&mut self.svc, &self.uad, item_key, uad_item);
        let uad_ship = self.uad.items.get_mut(item_key).get_ship_mut().unwrap();
        uad_ship.set_a_item_id(&self.uad.src, a_item_id);
        // Update on-fit ship kind
        let fit_key = uad_ship.get_fit_key();
        let ship_kind = uad_ship.get_kind();
        self.uad.fits.get_mut(fit_key).kind = ship_kind;
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::load_ship(&mut self.svc, &self.uad, item_key, uad_item);
    }
}

impl<'a> ShipMut<'a> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_ship_a_item_id(self.key, type_id)
    }
}
