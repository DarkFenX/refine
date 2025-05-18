use crate::sol::{ItemKey, SolarSystem, api::ShipMut, uad::item::ShipKind};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_ship(&mut self, item_key: ItemKey) {
        let uad_ship = self.uad.items.get(item_key).get_ship().unwrap();
        let fit_key = uad_ship.get_fit_key();
        // Remove incoming projections
        self.internal_remove_incoming_projections(item_key);
        // Remove ship from services
        SolarSystem::internal_remove_item_key_from_svc(&self.uad, &mut self.svc, item_key);
        // Remove ship from user data
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.ship = None;
        uad_fit.kind = ShipKind::Unknown;
        self.uad.items.remove(item_key);
    }
}

impl<'a> ShipMut<'a> {
    pub fn remove(self) {
        self.sol.internal_remove_ship(self.key);
    }
}
