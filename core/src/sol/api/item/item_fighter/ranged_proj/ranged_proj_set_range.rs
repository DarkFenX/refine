use itertools::Itertools;

use crate::{
    err::basic::ProjFoundError,
    misc::ProjRange,
    sol::SolarSystem,
    ud::{UItemKey, UProjRange},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fighter_proj_range(
        &mut self,
        item_key: UItemKey,
        projectee_key: UItemKey,
        range: ProjRange,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined before changing it
        let u_fighter = self.u_data.items.get(item_key).get_fighter().unwrap();
        let old_u_prange = u_fighter
            .get_projs()
            .get(&projectee_key)
            .ok_or_else(|| ProjFoundError {
                projector_item_id: u_fighter.get_item_id(),
                projectee_item_id: self.u_data.items.id_by_key(projectee_key),
            })?;
        let projectee_u_item = self.u_data.items.get(projectee_key);
        let u_prange = UProjRange::from_prange_with_axt(range, u_fighter.get_r_axt(), projectee_u_item.get_r_axt());
        // Do nothing if ranges are equal
        if u_prange == old_u_prange {
            return Ok(());
        }
        let autocharge_keys = u_fighter.get_autocharges().values().copied().collect_vec();
        // Update user data for fighter
        let u_fighter = self.u_data.items.get_mut(item_key).get_fighter_mut().unwrap();
        u_fighter.get_projs_mut().add(projectee_key, u_prange);
        // Update user data for autocharges
        for &autocharge_key in autocharge_keys.iter() {
            let u_autocharge = self.u_data.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            u_autocharge.get_projs_mut().add(projectee_key, u_prange);
        }
        // Update services for fighter
        let u_item = self.u_data.items.get(item_key);
        let projectee_u_item = self.u_data.items.get(projectee_key);
        SolarSystem::util_change_item_proj_range(
            &self.u_data,
            &mut self.svc,
            item_key,
            u_item,
            projectee_key,
            projectee_u_item,
            u_prange,
        );
        // Update services for autocharges
        for autocharge_key in autocharge_keys.into_iter() {
            let autocharge_u_item = self.u_data.items.get(autocharge_key);
            SolarSystem::util_change_item_proj_range(
                &self.u_data,
                &mut self.svc,
                autocharge_key,
                autocharge_u_item,
                projectee_key,
                projectee_u_item,
                u_prange,
            );
        }
        Ok(())
    }
}
