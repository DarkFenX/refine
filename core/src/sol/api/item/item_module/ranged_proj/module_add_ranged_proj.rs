use crate::{
    def::ItemId,
    err::basic::{ItemReceiveProjError, ProjNotFoundError},
    misc::ProjRange,
    sol::{
        SolarSystem,
        api::{AddRangedProjError, ModuleMut, RangedProjMut, get_r_ship_axt},
    },
    ud::{UItemKey, UProjRange},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_module_proj(
        &mut self,
        item_key: UItemKey,
        projectee_key: UItemKey,
        range: ProjRange,
    ) -> Result<(), AddRangedProjError> {
        // Check projector
        let u_item = self.u_data.items.get(item_key);
        let u_module = u_item.get_module().unwrap();
        // Check if projection has already been defined
        let projectee_u_item = self.u_data.items.get(projectee_key);
        if u_module.get_projs().contains(&projectee_key) {
            return Err(ProjNotFoundError {
                projector_item_id: u_module.get_item_id(),
                projectee_item_id: projectee_u_item.get_item_id(),
            }
            .into());
        }
        // Check if projectee can receive projections
        if !projectee_u_item.can_receive_projs() {
            return Err(ItemReceiveProjError {
                item_id: projectee_u_item.get_item_id(),
                item_kind: projectee_u_item.get_name(),
            }
            .into());
        }
        let u_prange = UProjRange::from_prange_with_axt(
            range,
            get_r_ship_axt(&self.u_data, u_module.get_fit_key()),
            projectee_u_item.get_r_axt(),
        );
        let charge_key = u_module.get_charge_key();
        // Update services for module
        SolarSystem::util_add_item_projection(
            &self.u_data,
            &mut self.svc,
            item_key,
            u_item,
            projectee_key,
            projectee_u_item,
            u_prange,
        );
        // Update services for charge
        if let Some(charge_key) = charge_key {
            let charge_u_item = self.u_data.items.get(charge_key);
            SolarSystem::util_add_item_projection(
                &self.u_data,
                &mut self.svc,
                charge_key,
                charge_u_item,
                projectee_key,
                projectee_u_item,
                u_prange,
            );
        }
        // Update user data for module
        let u_module = self.u_data.items.get_mut(item_key).get_module_mut().unwrap();
        u_module.get_projs_mut().add(projectee_key, u_prange);
        self.rev_projs.reg_projectee(item_key, projectee_key);
        // Update user data for charge
        if let Some(charge_key) = charge_key {
            let u_charge = self.u_data.items.get_mut(charge_key).get_charge_mut().unwrap();
            u_charge.get_projs_mut().add(projectee_key, u_prange);
            self.rev_projs.reg_projectee(charge_key, projectee_key);
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
        let projectee_key = self.sol.u_data.items.key_by_id_err(projectee_item_id)?;
        self.sol.internal_add_module_proj(self.key, projectee_key, range)?;
        Ok(RangedProjMut::new(self.sol, self.key, projectee_key))
    }
}
