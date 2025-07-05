use itertools::Itertools;

use crate::{def::ItemKey, err::basic::ProjFoundError, misc::ProjRange, sol::SolarSystem, uad::UadProjRange};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fighter_proj_range(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: ProjRange,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined before changing it
        let uad_fighter = self.uad.items.get(item_key).get_fighter().unwrap();
        let old_uad_prange = uad_fighter
            .get_projs()
            .get(&projectee_item_key)
            .ok_or_else(|| ProjFoundError {
                projector_item_id: uad_fighter.get_item_id(),
                projectee_item_id: self.uad.items.id_by_key(projectee_item_key),
            })?;
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        let uad_prange =
            UadProjRange::from_prange_with_extras(range, uad_fighter.get_a_extras(), projectee_uad_item.get_a_extras());
        // Do nothing if ranges are equal
        if uad_prange == old_uad_prange {
            return Ok(());
        }
        let autocharge_keys = uad_fighter.get_autocharges().values().copied().collect_vec();
        // Update user data for fighter
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        uad_fighter.get_projs_mut().add(projectee_item_key, uad_prange);
        // Update user data for autocharges
        for &autocharge_key in autocharge_keys.iter() {
            let uad_autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            uad_autocharge.get_projs_mut().add(projectee_item_key, uad_prange);
        }
        // Update services for fighter
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        SolarSystem::util_change_item_proj_range(
            &self.uad,
            &mut self.svc,
            &self.reffs,
            item_key,
            projectee_item_key,
            projectee_uad_item,
            uad_prange,
        );
        // Update services for autocharges
        for autocharge_key in autocharge_keys.into_iter() {
            SolarSystem::util_change_item_proj_range(
                &self.uad,
                &mut self.svc,
                &self.reffs,
                autocharge_key,
                projectee_item_key,
                projectee_uad_item,
                uad_prange,
            );
        }
        Ok(())
    }
}
