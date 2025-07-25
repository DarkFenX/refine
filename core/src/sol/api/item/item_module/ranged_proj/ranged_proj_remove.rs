use crate::{err::basic::ProjFoundError, sol::SolarSystem, ud::UItemKey};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_module_proj(
        &mut self,
        item_key: UItemKey,
        projectee_key: UItemKey,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined
        let u_item = self.u_data.items.get(item_key);
        let u_module = u_item.get_module().unwrap();
        if !u_module.get_projs().contains(&projectee_key) {
            return Err(ProjFoundError {
                projector_item_id: u_module.get_item_id(),
                projectee_item_id: self.u_data.items.id_by_key(projectee_key),
            });
        };
        let charge_key = u_module.get_charge_key();
        let projectee_u_item = self.u_data.items.get(projectee_key);
        // Update services for charge
        if let Some(charge_key) = charge_key {
            let charge_u_item = self.u_data.items.get(charge_key);
            SolarSystem::util_remove_item_projection(
                &self.u_data,
                &mut self.svc,
                charge_key,
                charge_u_item,
                projectee_key,
                projectee_u_item,
            );
        }
        // Update services for module
        SolarSystem::util_remove_item_projection(
            &self.u_data,
            &mut self.svc,
            item_key,
            u_item,
            projectee_key,
            projectee_u_item,
        );
        // Update user data for charge
        if let Some(charge_key) = charge_key {
            self.rev_projs.unreg_projectee(&charge_key, &projectee_key);
            let u_charge = self.u_data.items.get_mut(charge_key).get_charge_mut().unwrap();
            u_charge.get_projs_mut().remove(&projectee_key);
        }
        // Update user data for module
        self.rev_projs.unreg_projectee(&item_key, &projectee_key);
        let u_module = self.u_data.items.get_mut(item_key).get_module_mut().unwrap();
        u_module.get_projs_mut().remove(&projectee_key);
        Ok(())
    }
}
