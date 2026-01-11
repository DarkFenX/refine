use crate::{
    ad::AItemId,
    api::{Coordinates, FitMut, ItemTypeId, Movement, ShipMut},
    num::PValue,
    sol::SolarSystem,
    ud::{UEffectUpdates, UFitId, UItem, UItemId, UPhysics, UShip},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fit_ship(
        &mut self,
        fit_uid: UFitId,
        type_aid: AItemId,
        physics: UPhysics,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> UItemId {
        let u_fit = self.u_data.fits.get(fit_uid);
        // Remove old ship, if it was set
        if let Some(old_ship_uid) = u_fit.ship {
            self.internal_remove_ship(old_ship_uid, reuse_eupdates);
        }
        // Add new ship
        let item_id = self.u_data.items.alloc_id();
        let u_ship = UShip::new(item_id, type_aid, fit_uid, true, physics, &self.u_data.src);
        let ship_kind = u_ship.get_kind();
        let ship_radius = u_ship.get_axt().map(|v| v.radius).unwrap_or(PValue::ZERO);
        let u_item = UItem::Ship(u_ship);
        let ship_uid = self.u_data.items.add(u_item);
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.ship = Some(ship_uid);
        u_fit.ship_kind = ship_kind;
        SolarSystem::util_add_ship(&mut self.u_data, &mut self.svc, ship_uid, reuse_eupdates);
        // Update projections outgoing from on-ship items
        SolarSystem::util_update_ship_radius_for_outgoing_projs(&mut self.u_data, &mut self.svc, fit_uid, ship_radius);
        ship_uid
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
            u_physics.coordinates = coordinates.into_xyz();
        }
        if let Some(movement) = movement {
            u_physics.direction = movement.direction.into_xyz();
            u_physics.speed = movement.speed;
        }
        let mut reuse_eupdates = UEffectUpdates::new();
        let ship_uid = self
            .sol
            .internal_set_fit_ship(self.uid, type_id.into_aid(), u_physics, &mut reuse_eupdates);
        ShipMut::new(self.sol, ship_uid)
    }
}
