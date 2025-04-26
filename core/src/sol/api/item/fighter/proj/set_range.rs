use itertools::Itertools;

use crate::{
    err::basic::ProjFoundError,
    sol::{AttrVal, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_change_fighter_proj(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined before changing it
        let uad_fighter = self.uad.items.get(item_key).get_fighter().unwrap();
        let old_range = match uad_fighter.get_projs().get(&projectee_item_key) {
            Some(old_range) => *old_range,
            None => {
                return Err(ProjFoundError {
                    projector_item_id: uad_fighter.get_item_id(),
                    projectee_item_id: self.uad.items.id_by_key(projectee_item_key),
                });
            }
        };
        // Do nothing if ranges are equal
        if range == old_range {
            return Ok(());
        }
        // Update user data for fighter
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        uad_fighter.get_projs_mut().add(projectee_item_key, range);
        let autocharge_keys = uad_fighter.get_autocharges().values().copied().collect_vec();
        // Update services for fighter
        self.internal_change_item_key_projection_range_in_svc(item_key, projectee_item_key, range);
        for autocharge_key in autocharge_keys {
            // Update user data for autocharge
            let uad_autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            uad_autocharge.get_projs_mut().add(projectee_item_key, range);
            // Update services for autocharge
            self.internal_change_item_key_projection_range_in_svc(autocharge_key, projectee_item_key, range);
        }
        Ok(())
    }
}
