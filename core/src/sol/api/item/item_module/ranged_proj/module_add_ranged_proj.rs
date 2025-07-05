use crate::{
    def::{ItemId, ItemKey},
    err::basic::{ItemReceiveProjError, ProjNotFoundError},
    misc::ProjRange,
    sol::{
        SolarSystem,
        api::{AddRangedProjError, ModuleMut, RangedProjMut, get_ship_a_extras},
    },
    uad::UadProjRange,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_module_proj(
        &mut self,
        item_key: ItemKey,
        projectee_key: ItemKey,
        range: ProjRange,
    ) -> Result<(), AddRangedProjError> {
        // Check projector
        let uad_item = self.uad.items.get(item_key);
        let uad_module = uad_item.get_module().unwrap();
        // Check if projection has already been defined
        let projectee_uad_item = self.uad.items.get(projectee_key);
        if uad_module.get_projs().contains(&projectee_key) {
            return Err(ProjNotFoundError {
                projector_item_id: uad_module.get_item_id(),
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
        let uad_prange = UadProjRange::from_prange_with_extras(
            range,
            get_ship_a_extras(&self.uad, uad_module.get_fit_key()),
            projectee_uad_item.get_a_extras(),
        );
        let charge_key = uad_module.get_charge_key();
        // Update services for module
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
        // Update services for charge
        if let Some(charge_key) = charge_key {
            let charge_uad_item = self.uad.items.get(charge_key);
            SolarSystem::util_add_item_projection(
                &self.uad,
                &mut self.svc,
                &self.reffs,
                charge_key,
                charge_uad_item,
                projectee_key,
                projectee_uad_item,
                uad_prange,
            );
        }
        // Update user data for module
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        uad_module.get_projs_mut().add(projectee_key, uad_prange);
        self.rprojs.reg_projectee(item_key, projectee_key);
        // Update user data for charge
        if let Some(charge_key) = charge_key {
            let uad_charge = self.uad.items.get_mut(charge_key).get_charge_mut().unwrap();
            uad_charge.get_projs_mut().add(projectee_key, uad_prange);
            self.rprojs.reg_projectee(charge_key, projectee_key);
        }
        Ok(())
    }
}

impl<'a> ModuleMut<'a> {
    pub fn add_proj(
        &mut self,
        projectee_item_id: &ItemId,
        range: ProjRange,
    ) -> Result<RangedProjMut<'_>, AddRangedProjError> {
        let projectee_key = self.sol.uad.items.key_by_id_err(projectee_item_id)?;
        self.sol.internal_add_module_proj(self.key, projectee_key, range)?;
        Ok(RangedProjMut::new(self.sol, self.key, projectee_key))
    }
}
