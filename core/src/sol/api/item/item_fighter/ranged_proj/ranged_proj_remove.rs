use itertools::Itertools;

use crate::{err::basic::ProjFoundError, sol::SolarSystem, ud::UItemKey};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_fighter_proj(
        &mut self,
        fighter_key: UItemKey,
        projectee_key: UItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let u_fighter = self.u_data.items.get(fighter_key).dc_fighter().unwrap();
        if !u_fighter.get_projs().contains(&projectee_key) {
            return Err(ProjFoundError {
                projector_item_id: u_fighter.get_item_id(),
                projectee_item_id: self.u_data.items.id_by_key(projectee_key),
            });
        };
        let autocharge_keys = u_fighter.get_autocharges().values().collect_vec();
        // Update services for autocharge
        for &autocharge_key in autocharge_keys.iter() {
            SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, autocharge_key, projectee_key);
        }
        // Update services for fighter
        SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, fighter_key, projectee_key);
        // Update user data for autocharges
        for autocharge_key in autocharge_keys.into_iter() {
            self.rev_projs.unreg_projectee(&autocharge_key, projectee_key);
            let u_autocharge = self.u_data.items.get_mut(autocharge_key).dc_autocharge_mut().unwrap();
            u_autocharge.get_projs_mut().remove(&projectee_key);
        }
        // Update user data for fighter
        self.rev_projs.unreg_projectee(&fighter_key, projectee_key);
        let u_fighter = self.u_data.items.get_mut(fighter_key).dc_fighter_mut().unwrap();
        u_fighter.get_projs_mut().remove(&projectee_key);
        Ok(())
    }
}
