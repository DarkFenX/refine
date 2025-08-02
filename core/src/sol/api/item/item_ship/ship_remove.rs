use crate::{
    def::OF,
    sol::{SolarSystem, api::ShipMut},
    ud::{UEffectUpdates, UItemKey, UShipKind},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_ship(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_ship = self.u_data.items.get(item_key).get_ship().unwrap();
        let fit_key = u_ship.get_fit_key();
        // Remove incoming projections
        self.internal_remove_incoming_projections(item_key);
        // Remove ship from services
        SolarSystem::util_remove_ship(&mut self.u_data, &mut self.svc, item_key, reuse_eupdates);
        // Remove ship from user data
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.ship = None;
        u_fit.kind = UShipKind::Unknown;
        self.u_data.items.remove(item_key);
        // Update projections outgoing from on-ship items
        SolarSystem::util_update_ship_radius_for_outgoing_projs(&mut self.u_data, &mut self.svc, fit_key, OF(0.0));
    }
}

impl<'a> ShipMut<'a> {
    pub fn remove(self) {
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol.internal_remove_ship(self.key, &mut reuse_eupdates)
    }
}
