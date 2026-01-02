use crate::{
    ad::AItemId,
    api::{Coordinates, FitMut, Movement, ShipMut},
    def::{ItemTypeId, OF},
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitId, UItem, UItemId, UPhysics, UShip},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fit_ship(
        &mut self,
        fit_key: UFitId,
        type_id: AItemId,
        physics: UPhysics,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get(fit_key);
        // Remove old ship, if it was set
        if let Some(old_ship_key) = u_fit.ship {
            self.internal_remove_ship(old_ship_key, reuse_eupdates);
        }
        // Add new ship
        let item_id = self.u_data.items.alloc_id();
        let u_ship = UShip::new(item_id, type_id, fit_key, true, physics, &self.u_data.src);
        let ship_kind = u_ship.get_kind();
        let ship_radius = u_ship.get_axt().map(|v| v.radius).unwrap_or(OF(0.0));
        let u_item = UItem::Ship(u_ship);
        let ship_key = self.u_data.items.add(u_item);
        let u_fit = self.u_data.fits.get_mut(fit_key);
        u_fit.ship = Some(ship_key);
        u_fit.ship_kind = ship_kind;
        SolarSystem::util_add_ship(&mut self.u_data, &mut self.svc, ship_key, reuse_eupdates);
        // Update projections outgoing from on-ship items
        SolarSystem::util_update_ship_radius_for_outgoing_projs(&mut self.u_data, &mut self.svc, fit_key, ship_radius);
        ship_key
    }
}

impl<'a> FitMut<'a> {
    pub fn set_ship(
        &mut self,
        type_id: ItemTypeId,
        coordinates: Option<Coordinates>,
        movement: Option<Movement>,
    ) -> ShipMut<'_> {
        let mut u_physics = UPhysics::default();
        if let Some(coordinates) = coordinates {
            u_physics.coordinates = coordinates.into();
        }
        if let Some(movement) = movement {
            u_physics.direction = movement.direction.into();
            u_physics.speed = movement.speed;
        }
        let mut reuse_eupdates = UEffectUpdates::new();
        let ship_key = self
            .sol
            .internal_set_fit_ship(self.key, type_id, u_physics, &mut reuse_eupdates);
        ShipMut::new(self.sol, ship_key)
    }
}
