use itertools::Itertools;

use crate::{
    err::basic::ProjFoundError,
    sol::{ItemKey, SolarSystem},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_remove_fighter_proj(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let uad_fighter = self.uad.items.get(item_key).get_fighter().unwrap();
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        if !uad_fighter.get_projs().contains(&projectee_item_key) {
            return Err(ProjFoundError {
                projector_item_id: uad_fighter.get_item_id(),
                projectee_item_id: projectee_uad_item.get_item_id(),
            });
        };
        let autocharge_keys = uad_fighter.get_autocharges().values().copied().collect_vec();
        for autocharge_key in autocharge_keys {
            // Update services for autocharge
            self.internal_remove_item_key_projection_from_svc(autocharge_key, projectee_item_key);
            // Update user data for autocharge
            self.proj_tracker.unreg_projectee(&autocharge_key, &projectee_item_key);
            let uad_autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            uad_autocharge.get_projs_mut().remove(&projectee_item_key);
        }
        // Update services for fighter
        self.internal_remove_item_key_projection_from_svc(item_key, projectee_item_key);
        // Update user data for fighter
        self.proj_tracker.unreg_projectee(&item_key, &projectee_item_key);
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        uad_fighter.get_projs_mut().remove(&projectee_item_key);
        Ok(())
    }
}
