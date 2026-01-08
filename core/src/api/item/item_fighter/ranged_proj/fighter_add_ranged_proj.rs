use itertools::Itertools;

use crate::{
    api::{AddProjError, FighterMut, ProjMut},
    err::basic::{ItemReceiveProjError, ProjNotFoundError},
    sol::SolarSystem,
    ud::{ItemId, UItemId, UProjData},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_fighter_proj(
        &mut self,
        fighter_uid: UItemId,
        projectee_uid: UItemId,
    ) -> Result<(), AddProjError> {
        // Check projector
        let u_fighter = self.u_data.items.get(fighter_uid).dc_fighter().unwrap();
        // Check if projection has already been defined
        let projectee_u_item = self.u_data.items.get(projectee_uid);
        if u_fighter.get_projs().contains(&projectee_uid) {
            return Err(ProjNotFoundError {
                projector_item_id: u_fighter.get_item_id(),
                projectee_item_id: projectee_u_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections by getting its user physics
        let projectee_physics = match projectee_u_item.get_direct_physics() {
            Some(projectee_physics) => *projectee_physics,
            None => {
                return Err(ItemReceiveProjError {
                    item_id: projectee_u_item.get_item_id(),
                    item_kind: projectee_u_item.lib_get_name(),
                }
                .into());
            }
        };
        let fighter_physics = *u_fighter.get_physics();
        let u_proj_data = Some(UProjData::from_physics_with_axt(
            fighter_physics,
            projectee_physics,
            u_fighter.get_axt(),
            projectee_u_item.get_axt(),
        ));
        let autocharge_uids = u_fighter.get_autocharges().values().collect_vec();
        // Update user data for fighter
        let u_fighter = self.u_data.items.get_mut(fighter_uid).dc_fighter_mut().unwrap();
        u_fighter.get_projs_mut().add(projectee_uid, u_proj_data);
        self.rev_projs.reg_projectee(fighter_uid, projectee_uid);
        // Update services for fighte
        SolarSystem::util_add_item_projection(&self.u_data, &mut self.svc, fighter_uid, projectee_uid, u_proj_data);
        for autocharge_uid in autocharge_uids {
            // Update user data for autocharge
            let u_autocharge = self.u_data.items.get_mut(autocharge_uid).dc_autocharge_mut().unwrap();
            u_autocharge.get_projs_mut().add(projectee_uid, u_proj_data);
            self.rev_projs.reg_projectee(autocharge_uid, projectee_uid);
            // Update services for autocharge
            SolarSystem::util_add_item_projection(
                &self.u_data,
                &mut self.svc,
                autocharge_uid,
                projectee_uid,
                u_proj_data,
            );
        }
        Ok(())
    }
}

impl<'a> FighterMut<'a> {
    pub fn add_proj(&mut self, projectee_item_id: &ItemId) -> Result<ProjMut<'_>, AddProjError> {
        let projectee_uid = self.sol.u_data.items.iid_by_xid_err(projectee_item_id)?;
        self.sol.internal_add_fighter_proj(self.uid, projectee_uid)?;
        Ok(ProjMut::new(self.sol, self.uid, projectee_uid))
    }
}
