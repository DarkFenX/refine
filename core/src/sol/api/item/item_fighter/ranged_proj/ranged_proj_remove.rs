use itertools::Itertools;

use crate::{err::basic::ProjFoundError, sol::SolarSystem, ud::UItemKey};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_fighter_proj(
        &mut self,
        item_key: UItemKey,
        projectee_key: UItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let u_item = self.u_data.items.get(item_key);
        let u_fighter = u_item.get_fighter().unwrap();
        let projectee_u_item = self.u_data.items.get(projectee_key);
        if !u_fighter.get_projs().contains(&projectee_key) {
            return Err(ProjFoundError {
                projector_item_id: u_fighter.get_item_id(),
                projectee_item_id: projectee_u_item.get_item_id(),
            });
        };
        let autocharge_keys = u_fighter.get_autocharges().values().copied().collect_vec();
        // Update services for autocharge
        for &autocharge_key in autocharge_keys.iter() {
            let autocharge_u_item = self.u_data.items.get(autocharge_key);
            SolarSystem::util_remove_item_projection(
                &self.u_data,
                &mut self.svc,
                autocharge_key,
                autocharge_u_item,
                projectee_key,
                projectee_u_item,
            );
        }
        // Update services for fighter
        SolarSystem::util_remove_item_projection(
            &self.u_data,
            &mut self.svc,
            item_key,
            u_item,
            projectee_key,
            projectee_u_item,
        );
        // Update user data for autocharges
        for autocharge_key in autocharge_keys.into_iter() {
            self.rev_projs.unreg_projectee(&autocharge_key, &projectee_key);
            let u_autocharge = self.u_data.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            u_autocharge.get_projs_mut().remove(&projectee_key);
        }
        // Update user data for fighter
        self.rev_projs.unreg_projectee(&item_key, &projectee_key);
        let u_fighter = self.u_data.items.get_mut(item_key).get_fighter_mut().unwrap();
        u_fighter.get_projs_mut().remove(&projectee_key);
        Ok(())
    }
}
