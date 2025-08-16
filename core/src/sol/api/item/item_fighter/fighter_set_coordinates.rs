use itertools::Itertools;

use crate::{
    misc::Coordinates,
    sol::{SolarSystem, api::FighterMut},
    ud::{UCoordinates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fighter_coordinates(
        &mut self,
        item_key: UItemKey,
        u_coordinates: UCoordinates,
    ) {
        let u_fighter = self.u_data.items.get_mut(item_key).get_fighter_mut().unwrap();
        if u_fighter.get_position().coordinates == u_coordinates {
            return;
        }
        u_fighter.get_pos_mut().coordinates = u_coordinates;
        let u_fighter_pos = *u_fighter.get_position();
        if !u_fighter.get_projs_mut().is_empty() {
            // Handle outgoing projections for fighter itself
            for u_proj_data in u_fighter.get_projs_mut().iter_datas_mut() {
                u_proj_data.update_src_pos(u_fighter_pos);
            }
            let u_fighter = self.u_data.items.get(item_key).get_fighter().unwrap();
            for (projectee_key, u_proj_data) in u_fighter.get_projs().iter_projectees_and_datas() {
                SolarSystem::util_change_item_proj_data(
                    &self.u_data,
                    &mut self.svc,
                    item_key,
                    projectee_key,
                    Some(u_proj_data),
                );
            }
            // Handle outgoing projections for autocharges itself
            let autocharge_keys = u_fighter.get_autocharges().values().collect_vec();
            for autocharge_key in autocharge_keys {
                let u_autocharge = self.u_data.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
                for u_proj_data in u_autocharge.get_projs_mut().iter_datas_mut() {
                    u_proj_data.update_src_pos(u_fighter_pos);
                }
                let u_autocharge = self.u_data.items.get(item_key).get_autocharge().unwrap();
                for (projectee_key, u_proj_data) in u_autocharge.get_projs().iter_projectees_and_datas() {
                    SolarSystem::util_change_item_proj_data(
                        &self.u_data,
                        &mut self.svc,
                        item_key,
                        projectee_key,
                        Some(u_proj_data),
                    );
                }
            }
        }
        // Handle incoming projections
        SolarSystem::util_update_position_for_incoming(
            &mut self.u_data,
            &self.rev_projs,
            &mut self.svc,
            item_key,
            u_fighter_pos,
        );
    }
}

impl<'a> FighterMut<'a> {
    /// Set fighter position in its solar system.
    pub fn set_coordinates(&mut self, coordinates: Coordinates) {
        self.sol.internal_set_fighter_coordinates(self.key, coordinates.into())
    }
}
