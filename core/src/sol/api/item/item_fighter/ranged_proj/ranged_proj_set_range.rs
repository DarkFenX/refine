use itertools::Itertools;

use crate::{
    err::basic::ProjFoundError,
    misc::ProjRange,
    sol::SolarSystem,
    uad::{UadItemKey, UadProjRange},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fighter_proj_range(
        &mut self,
        item_key: UadItemKey,
        projectee_key: UadItemKey,
        range: ProjRange,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined before changing it
        let uad_fighter = self.uad.items.get(item_key).get_fighter().unwrap();
        let old_uad_prange = uad_fighter
            .get_projs()
            .get(&projectee_key)
            .ok_or_else(|| ProjFoundError {
                projector_item_id: uad_fighter.get_item_id(),
                projectee_item_id: self.uad.items.id_by_key(projectee_key),
            })?;
        let projectee_uad_item = self.uad.items.get(projectee_key);
        let uad_prange =
            UadProjRange::from_prange_with_axt(range, uad_fighter.get_r_axt(), projectee_uad_item.get_r_axt());
        // Do nothing if ranges are equal
        if uad_prange == old_uad_prange {
            return Ok(());
        }
        let autocharge_keys = uad_fighter.get_autocharges().values().copied().collect_vec();
        // Update user data for fighter
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        uad_fighter.get_projs_mut().add(projectee_key, uad_prange);
        // Update user data for autocharges
        for &autocharge_key in autocharge_keys.iter() {
            let uad_autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            uad_autocharge.get_projs_mut().add(projectee_key, uad_prange);
        }
        // Update services for fighter
        let uad_item = self.uad.items.get(item_key);
        let projectee_uad_item = self.uad.items.get(projectee_key);
        SolarSystem::util_change_item_proj_range(
            &self.uad,
            &mut self.svc,
            item_key,
            uad_item,
            projectee_key,
            projectee_uad_item,
            uad_prange,
        );
        // Update services for autocharges
        for autocharge_key in autocharge_keys.into_iter() {
            let autocharge_uad_item = self.uad.items.get(autocharge_key);
            SolarSystem::util_change_item_proj_range(
                &self.uad,
                &mut self.svc,
                autocharge_key,
                autocharge_uad_item,
                projectee_key,
                projectee_uad_item,
                uad_prange,
            );
        }
        Ok(())
    }
}
