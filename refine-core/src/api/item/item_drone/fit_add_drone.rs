use crate::{
    ad::AItemId,
    api::{Coordinates, DroneMut, FitMut, ItemTypeId, MinionState, Movement},
    sol::{SolarSystem, UndoStep},
    ud::{UDrone, UFitId, UItem, UItemId, UItemMutationRequest, UPhysics},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_drone(
        &mut self,
        fit_uid: UFitId,
        type_aid: AItemId,
        state: MinionState,
        mutation: Option<UItemMutationRequest>,
        physics: UPhysics,
    ) -> UItemId {
        let item_id = self.u_data.items.alloc_id();
        let u_drone = UDrone::new(
            item_id,
            type_aid,
            fit_uid,
            state,
            mutation,
            physics,
            &self.u_data.r_data,
        );
        let u_item = UItem::Drone(u_drone);
        let drone_uid = self.u_data.items.add(u_item);
        self.attach_drone(fit_uid, drone_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::DroneAdded(drone_uid));
        }
        drone_uid
    }
    pub(in crate::api) fn internal_reattach_drone(&mut self, drone_uid: UItemId) {
        self.u_data.items.reattach(drone_uid);
        let u_drone = self.u_data.items.get(drone_uid).dc_drone().unwrap();
        let fit_uid = u_drone.get_fit_uid();
        self.attach_drone(fit_uid, drone_uid);
    }
    fn attach_drone(&mut self, fit_uid: UFitId, drone_uid: UItemId) {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.drones.insert(drone_uid);
        SolarSystem::util_add_drone(&mut self.u_data, &mut self.svc, drone_uid, &mut self.cache.eupdates);
        SolarSystem::util_reg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, drone_uid);
    }
}

impl<'s> FitMut<'s> {
    pub fn add_drone(
        &mut self,
        type_id: ItemTypeId,
        state: MinionState,
        coordinates: Option<Coordinates>,
        movement: Option<Movement>,
    ) -> DroneMut<'_> {
        let mut u_physics = UPhysics::default();
        if let Some(coordinates) = coordinates {
            u_physics.coordinates = coordinates.into_xyz();
        }
        if let Some(movement) = movement {
            u_physics.direction = movement.direction.into_xyz();
            u_physics.speed = movement.speed;
        }
        let drone_uid = self
            .sol
            .internal_add_drone(self.uid, type_id.into_aid(), state, None, u_physics);
        DroneMut::new(self.sol, drone_uid)
    }
}
