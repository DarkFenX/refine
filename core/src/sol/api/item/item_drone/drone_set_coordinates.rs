use crate::{
    misc::Coordinates,
    sol::{SolarSystem, api::DroneMut},
    ud::{UCoordinates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_coordinates(
        &mut self,
        item_key: UItemKey,
        u_coordinates: UCoordinates,
    ) {
        let u_drone = self.u_data.items.get_mut(item_key).get_drone_mut().unwrap();
        if u_drone.get_pos().coordinates == u_coordinates {
            return;
        }
        u_drone.get_pos_mut().coordinates = u_coordinates;
        let u_drone_pos = *u_drone.get_pos();
        // Handle outgoing projections
        if !u_drone.get_projs_mut().is_empty() {
            for u_proj_data in u_drone.get_projs_mut().iter_datas_mut() {
                u_proj_data.update_src_pos(u_drone_pos);
            }
            let u_drone = self.u_data.items.get(item_key).get_drone().unwrap();
            for (projectee_key, u_proj_data) in u_drone.get_projs().iter_projectees_and_datas() {
                SolarSystem::util_change_item_proj_data(
                    &self.u_data,
                    &mut self.svc,
                    item_key,
                    projectee_key,
                    Some(u_proj_data),
                );
            }
        }
        // Handle incoming projections
        SolarSystem::util_update_position_for_incoming(
            &mut self.u_data,
            &self.rev_projs,
            &mut self.svc,
            item_key,
            u_drone_pos,
        );
    }
}

impl<'a> DroneMut<'a> {
    /// Set drone position in its solar system.
    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.sol.internal_set_drone_coordinates(self.key, coordinates.into())
    }
}
