use crate::{err::basic::ProjFoundError, sol::SolarSystem, ud::UItemId};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_module_proj(
        &mut self,
        module_uid: UItemId,
        projectee_uid: UItemId,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let u_module = self.u_data.items.get(module_uid).dc_module().unwrap();
        if !u_module.get_projs().contains(&projectee_uid) {
            return Err(ProjFoundError {
                projector_item_id: u_module.get_item_id(),
                projectee_item_id: self.u_data.items.xid_by_iid(projectee_uid),
            });
        };
        let charge_uid = u_module.get_charge_uid();
        // Update services for charge
        if let Some(charge_uid) = charge_uid {
            SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, charge_uid, projectee_uid);
        }
        // Update services for module
        SolarSystem::util_remove_item_projection(&self.u_data, &mut self.svc, module_uid, projectee_uid);
        // Update user data for charge
        if let Some(charge_uid) = charge_uid {
            self.rev_projs.unreg_projectee(&charge_uid, projectee_uid);
            let u_charge = self.u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
            u_charge.get_projs_mut().remove(&projectee_uid);
        }
        // Update user data for module
        self.rev_projs.unreg_projectee(&module_uid, projectee_uid);
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.get_projs_mut().remove(&projectee_uid);
        Ok(())
    }
}
