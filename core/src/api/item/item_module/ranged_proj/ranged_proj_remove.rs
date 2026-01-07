use crate::{err::basic::ProjFoundError, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_module_proj(
        &mut self,
        module_key: UItemId,
        projectee_key: UItemId,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let u_module = self.u_data.items.get(module_key).dc_module().unwrap();
        if !u_module.get_projs().contains(&projectee_key) {
            return Err(ProjFoundError {
                projector_item_id: u_module.get_item_id(),
                projectee_item_id: self.u_data.items.xid_by_iid(projectee_key),
            });
        };
        let charge_key = u_module.get_charge_uid();
        // Update services for charge
        if let Some(charge_key) = charge_key {
            SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, charge_key, projectee_key);
        }
        // Update services for module
        SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, module_key, projectee_key);
        // Update user data for charge
        if let Some(charge_key) = charge_key {
            self.rev_projs.unreg_projectee(&charge_key, projectee_key);
            let u_charge = self.u_data.items.get_mut(charge_key).dc_charge_mut().unwrap();
            u_charge.get_projs_mut().remove(&projectee_key);
        }
        // Update user data for module
        self.rev_projs.unreg_projectee(&module_key, projectee_key);
        let u_module = self.u_data.items.get_mut(module_key).dc_module_mut().unwrap();
        u_module.get_projs_mut().remove(&projectee_key);
        Ok(())
    }
}
