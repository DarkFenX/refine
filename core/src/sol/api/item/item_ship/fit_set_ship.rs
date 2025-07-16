use crate::{
    ad,
    def::{FitKey, ItemKey, ItemTypeId, OF},
    sol::{
        SolarSystem,
        api::{FitMut, ShipMut},
    },
    uad::{UadEffectUpdates, UadItem, UadShip},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_ship(
        &mut self,
        fit_key: FitKey,
        a_item_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> ItemKey {
        let uad_fit = self.uad.fits.get(fit_key);
        // Remove old ship, if it was set
        if let Some(old_item_key) = uad_fit.ship {
            self.internal_remove_ship(old_item_key, reuse_eupdates);
        }
        // Add new ship
        let item_id = self.uad.items.alloc_id();
        let uad_ship = UadShip::new(item_id, a_item_id, fit_key, true, &self.uad.src, reuse_eupdates);
        let ship_kind = uad_ship.get_kind();
        let ship_radius = uad_ship.get_a_xt().map(|v| v.radius).unwrap_or(OF(0.0));
        let uad_item = UadItem::Ship(uad_ship);
        let item_key = self.uad.items.add(uad_item);
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.ship = Some(item_key);
        uad_fit.kind = ship_kind;
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_ship(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        // Update projections outgoing from on-ship items
        SolarSystem::util_update_ship_radius_for_outgoing_projs(&mut self.uad, &mut self.svc, fit_key, ship_radius);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn set_ship(&mut self, type_id: ItemTypeId) -> ShipMut<'_> {
        let mut reuse_eupdates = UadEffectUpdates::new();
        let item_key = self.sol.internal_set_fit_ship(self.key, type_id, &mut reuse_eupdates);
        ShipMut::new(self.sol, item_key)
    }
}
