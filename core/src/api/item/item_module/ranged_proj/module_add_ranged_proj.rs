use crate::{
    api::{AddProjError, ModuleMut, RangedProjMut, get_ship_axt},
    def::ItemId,
    err::basic::{ItemReceiveProjError, ProjNotFoundError},
    sol::SolarSystem,
    ud::{UItemId, UProjData},
};

impl SolarSystem {
    pub(in crate::api) fn internal_add_module_proj(
        &mut self,
        module_key: UItemId,
        projectee_key: UItemId,
    ) -> Result<(), AddProjError> {
        // Check projector
        let u_item = self.u_data.items.get(module_key);
        let u_module = u_item.dc_module().unwrap();
        // Check if projection has already been defined
        let projectee_u_item = self.u_data.items.get(projectee_key);
        if u_module.get_projs().contains(&projectee_key) {
            return Err(ProjNotFoundError {
                projector_item_id: u_module.get_item_id(),
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
        let ship_physics = self.u_data.get_ship_physics_by_fit_key(u_module.get_fit_uid());
        let u_proj_data = Some(UProjData::from_physics_with_axt(
            ship_physics,
            projectee_physics,
            get_ship_axt(&self.u_data, u_module.get_fit_uid()),
            projectee_u_item.get_axt(),
        ));
        let charge_key = u_module.get_charge_uid();
        // Update user data for module
        let u_module = self.u_data.items.get_mut(module_key).dc_module_mut().unwrap();
        u_module.get_projs_mut().add(projectee_key, u_proj_data);
        self.rev_projs.reg_projectee(module_key, projectee_key);
        // Update user data for charge
        if let Some(charge_key) = charge_key {
            let u_charge = self.u_data.items.get_mut(charge_key).dc_charge_mut().unwrap();
            u_charge.get_projs_mut().add(projectee_key, u_proj_data);
            self.rev_projs.reg_projectee(charge_key, projectee_key);
        }
        // Update services for module
        SolarSystem::util_add_item_projection(&self.u_data, &mut self.svc, module_key, projectee_key, u_proj_data);
        // Update services for charge
        if let Some(charge_key) = charge_key {
            SolarSystem::util_add_item_projection(&self.u_data, &mut self.svc, charge_key, projectee_key, u_proj_data);
        }
        Ok(())
    }
}

impl<'a> ModuleMut<'a> {
    pub fn add_proj(&mut self, projectee_item_id: &ItemId) -> Result<RangedProjMut<'_>, AddProjError> {
        let projectee_key = self.sol.u_data.items.iid_by_eid_err(projectee_item_id)?;
        self.sol.internal_add_module_proj(self.key, projectee_key)?;
        Ok(RangedProjMut::new(self.sol, self.key, projectee_key))
    }
}
