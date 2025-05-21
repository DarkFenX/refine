use itertools::Itertools;

use crate::{
    err::basic::ProjFoundError,
    sol::{ItemKey, SolarSystem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_fighter_proj(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let uad_item = self.uad.items.get(item_key);
        let uad_fighter = uad_item.get_fighter().unwrap();
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        if !uad_fighter.get_projs().contains(&projectee_item_key) {
            return Err(ProjFoundError {
                projector_item_id: uad_fighter.get_item_id(),
                projectee_item_id: projectee_uad_item.get_item_id(),
            });
        };
        let autocharge_keys = uad_fighter.get_autocharges().values().copied().collect_vec();
        // Update services for autocharge
        for &autocharge_key in autocharge_keys.iter() {
            let autocharge_uad_item = self.uad.items.get(autocharge_key);
            SolarSystem::util_remove_item_projection(
                &self.uad,
                &mut self.svc,
                &self.reffs,
                autocharge_key,
                autocharge_uad_item,
                projectee_item_key,
                projectee_uad_item,
            );
        }
        // Update services for fighter
        SolarSystem::util_remove_item_projection(
            &self.uad,
            &mut self.svc,
            &self.reffs,
            item_key,
            uad_item,
            projectee_item_key,
            projectee_uad_item,
        );
        // Update user data for autocharges
        for autocharge_key in autocharge_keys.into_iter() {
            self.rprojs.unreg_projectee(&autocharge_key, &projectee_item_key);
            let uad_autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            uad_autocharge.get_projs_mut().remove(&projectee_item_key);
        }
        // Update user data for fighter
        self.rprojs.unreg_projectee(&item_key, &projectee_item_key);
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        uad_fighter.get_projs_mut().remove(&projectee_item_key);
        Ok(())
    }
}
