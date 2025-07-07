use itertools::Itertools;

use crate::{
    def::{ItemId, ItemKey},
    err::basic::{ItemReceiveProjError, ProjNotFoundError},
    misc::ProjRange,
    sol::{
        SolarSystem,
        api::{AddRangedProjError, FighterMut, ProjMut},
    },
    uad::UadProjRange,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_fighter_proj(
        &mut self,
        item_key: ItemKey,
        projectee_key: ItemKey,
        range: ProjRange,
    ) -> Result<(), AddRangedProjError> {
        // Check projector
        let uad_fighter = self.uad.items.get(item_key).get_fighter().unwrap();
        // Check if projection has already been defined
        let projectee_uad_item = self.uad.items.get(projectee_key);
        if uad_fighter.get_projs().contains(&projectee_key) {
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
        let uad_prange =
            UadProjRange::from_prange_with_xt(range, uad_fighter.get_a_xt(), projectee_uad_item.get_a_xt());
        let autocharge_keys = uad_fighter.get_autocharges().values().copied().collect_vec();
        // Update user data for fighter
        let uad_fighter = self.uad.items.get_mut(item_key).get_fighter_mut().unwrap();
        uad_fighter.get_projs_mut().add(projectee_key, uad_prange);
        self.rprojs.reg_projectee(item_key, projectee_key);
        // Update services for fighter
        let uad_item = self.uad.items.get(item_key);
        let projectee_uad_item = self.uad.items.get(projectee_key);
        SolarSystem::util_add_item_projection(
            &self.uad,
            &mut self.svc,
            &self.reffs,
            item_key,
            uad_item,
            projectee_key,
            projectee_uad_item,
            uad_prange,
        );
        for autocharge_key in autocharge_keys {
            // Update user data for autocharge
            let uad_autocharge = self.uad.items.get_mut(autocharge_key).get_autocharge_mut().unwrap();
            uad_autocharge.get_projs_mut().add(projectee_key, uad_prange);
            self.rprojs.reg_projectee(autocharge_key, projectee_key);
            // Update services for autocharge
            let autocharge_uad_item = self.uad.items.get(autocharge_key);
            let projectee_uad_item = self.uad.items.get(projectee_key);
            SolarSystem::util_add_item_projection(
                &self.uad,
                &mut self.svc,
                &self.reffs,
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

impl<'a> FighterMut<'a> {
    pub fn add_proj(
        &mut self,
        projectee_item_id: &ItemId,
        range: ProjRange,
    ) -> Result<ProjMut<'_>, AddRangedProjError> {
        let projectee_key = self.sol.uad.items.key_by_id_err(projectee_item_id)?;
        self.sol.internal_add_fighter_proj(self.key, projectee_key, range)?;
        Ok(ProjMut::new(self.sol, self.key, projectee_key))
    }
}
