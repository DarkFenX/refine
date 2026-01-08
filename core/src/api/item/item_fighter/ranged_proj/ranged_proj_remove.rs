use itertools::Itertools;

use crate::{err::basic::ProjFoundError, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_fighter_proj(
        &mut self,
        fighter_uid: UItemId,
        projectee_uid: UItemId,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let u_fighter = self.u_data.items.get(fighter_uid).dc_fighter().unwrap();
        if !u_fighter.get_projs().contains(&projectee_uid) {
            return Err(ProjFoundError {
                projector_item_id: u_fighter.get_item_id(),
                projectee_item_id: self.u_data.items.xid_by_iid(projectee_uid),
            });
        };
        let autocharge_uids = u_fighter.get_autocharges().values().collect_vec();
        // Update services for autocharge
        for &autocharge_uid in autocharge_uids.iter() {
            SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, autocharge_uid, projectee_uid);
        }
        // Update services for fighter
        SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, fighter_uid, projectee_uid);
        // Update user data for autocharges
        for autocharge_uid in autocharge_uids.into_iter() {
            self.rev_projs.unreg_projectee(&autocharge_uid, projectee_uid);
            let u_autocharge = self.u_data.items.get_mut(autocharge_uid).dc_autocharge_mut().unwrap();
            u_autocharge.get_projs_mut().remove(&projectee_uid);
        }
        // Update user data for fighter
        self.rev_projs.unreg_projectee(&fighter_uid, projectee_uid);
        let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        u_fighter.get_projs_mut().remove(&projectee_uid);
        Ok(())
    }
}
