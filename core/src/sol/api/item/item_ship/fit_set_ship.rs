use crate::{
    ad,
    sol::{
        FitKey, ItemKey, ItemTypeId, SolarSystem,
        api::{FitMut, ShipMut},
        uad::item::{UadItem, UadShip},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fit_ship(&mut self, fit_key: FitKey, a_item_id: ad::AItemId) -> ItemKey {
        let uad_fit = self.uad.fits.get(fit_key);
        // Remove old ship, if it was set
        if let Some(old_item_key) = uad_fit.ship {
            self.internal_remove_ship(old_item_key);
        }
        // Add new ship
        let item_id = self.uad.items.alloc_id();
        let uad_ship = UadShip::new(&self.uad.src, item_id, a_item_id, fit_key, true);
        let ship_kind = uad_ship.get_kind();
        let uad_item = UadItem::Ship(uad_ship);
        let item_key = self.uad.items.add(uad_item);
        let uad_fit = self.uad.fits.get_mut(fit_key);
        uad_fit.ship = Some(item_key);
        uad_fit.kind = ship_kind;
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_ship(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        item_key
    }
}

impl<'a> FitMut<'a> {
    pub fn set_ship(&mut self, type_id: ItemTypeId) -> ShipMut<'_> {
        let item_key = self.sol.internal_set_fit_ship(self.key, type_id);
        ShipMut::new(self.sol, item_key)
    }
}
