use itertools::Itertools;

use crate::{
    err::basic::{ItemReceiveProjError, ProjNotFoundError},
    sol::{
        AttrVal, ItemId, ItemKey, SolarSystem,
        api::{AddRangedProjError, FighterMut, ProjMut},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_fighter_proj(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), AddRangedProjError> {
        // Check projector
        let uad_fighter = self.uad.items.get(item_key).get_fighter().unwrap();
        // Check if projection has already been defined
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        if uad_fighter.get_projs().contains(&projectee_item_key) {
            return Err(ProjNotFoundError {
                projector_item_id: uad_fighter.get_item_id(),
                projectee_item_id: projectee_uad_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections
        if !projectee_uad_item.can_receive_projs() {
            return Err(ItemReceiveProjError {
                item_id: projectee_uad_item.get_item_id(),
                item_kind: projectee_uad_item.get_name(),
            }
            .into());
        }
        // Update user data for fighter
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        let autocharge_keys = uad_fighter.get_autocharges().values().copied().collect_vec();
        uad_fighter.get_projs_mut().add(projectee_item_key, range);
        self.proj_tracker.reg_projectee(item_key, projectee_item_key);
        // Update services for fighter
        SolarSystem::internal_add_item_key_projection_to_svc(
            &self.uad,
            &mut self.svc,
            item_key,
            projectee_item_key,
            range,
        );
        for autocharge_key in autocharge_keys {
            // Update user data for autocharge
            let uad_autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            uad_autocharge.get_projs_mut().add(projectee_item_key, range);
            self.proj_tracker.reg_projectee(autocharge_key, projectee_item_key);
            // Update services for autocharge
            SolarSystem::internal_add_item_key_projection_to_svc(
                &self.uad,
                &mut self.svc,
                autocharge_key,
                projectee_item_key,
                range,
            );
        }
        Ok(())
    }
}

impl<'a> FighterMut<'a> {
    pub fn add_proj(
        &mut self,
        projectee_item_id: &ItemId,
        range: Option<AttrVal>,
    ) -> Result<ProjMut, AddRangedProjError> {
        let projectee_item_key = self.sol.uad.items.key_by_id_err(projectee_item_id)?;
        self.sol
            .internal_add_fighter_proj(self.key, projectee_item_key, range)?;
        Ok(ProjMut::new(self.sol, self.key, projectee_item_key))
    }
}
