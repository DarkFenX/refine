use crate::{
    misc::Coordinates,
    sol::{SolarSystem, api::ShipMut},
    ud::{UCoordinates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_ship_coordinates(
        &mut self,
        ship_key: UItemKey,
        u_coordinates: UCoordinates,
    ) {
        let u_ship = self.u_data.items.get_mut(ship_key).get_ship_mut().unwrap();
        if u_ship.get_pos().coordinates == u_coordinates {
            return;
        }
        u_ship.get_pos_mut().coordinates = u_coordinates;
        let u_ship_pos = *u_ship.get_pos();
        // Handle outgoing projections
        let mut projections_to_update = Vec::new();
        for module_key in self.u_data.fits.get(u_ship.get_fit_key()).iter_module_keys() {
            let u_module = self.u_data.items.get_mut(module_key).get_module_mut().unwrap();
            for (projectee_key, u_proj_data) in u_module.get_projs_mut().iter_projectees_and_datas_mut() {
                u_proj_data.update_src_pos(u_ship_pos);
                projections_to_update.push((module_key, projectee_key, *u_proj_data));
            }
            if let Some(charge_key) = u_module.get_charge_key() {
                let u_charge = self.u_data.items.get_mut(charge_key).get_charge_mut().unwrap();
                for (projectee_key, u_proj_data) in u_charge.get_projs_mut().iter_projectees_and_datas_mut() {
                    u_proj_data.update_src_pos(u_ship_pos);
                    projections_to_update.push((charge_key, projectee_key, *u_proj_data));
                }
            }
        }
        for (projector_key, projectee_key, proj_data) in projections_to_update {
            SolarSystem::util_change_item_proj_data(
                &self.u_data,
                &mut self.svc,
                projector_key,
                projectee_key,
                Some(proj_data),
            );
        }
        // Handle incoming projections
        SolarSystem::util_update_position_for_incoming(
            &mut self.u_data,
            &self.rev_projs,
            &mut self.svc,
            ship_key,
            u_ship_pos,
        );
    }
}

impl<'a> ShipMut<'a> {
    /// Set ship position in its solar system.
    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.sol.internal_set_ship_coordinates(self.key, coordinates.into())
    }
}
