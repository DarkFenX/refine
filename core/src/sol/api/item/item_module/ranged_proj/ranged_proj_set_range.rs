use crate::{
    err::basic::ProjFoundError,
    misc::ProjRange,
    sol::{SolarSystem, api::get_r_ship_axt},
    ud::{UItemKey, UProjRange},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_proj_range(
        &mut self,
        item_key: UItemKey,
        projectee_key: UItemKey,
        range: ProjRange,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined before changing it
        let u_module = self.u_data.items.get(item_key).get_module().unwrap();
        let old_u_prange = u_module.get_projs().get(&projectee_key).ok_or_else(|| ProjFoundError {
            projector_item_id: u_module.get_item_id(),
            projectee_item_id: self.u_data.items.id_by_key(projectee_key),
        })?;
        let u_prange = UProjRange::from_prange_with_axt(
            range,
            get_r_ship_axt(&self.u_data, u_module.get_fit_key()),
            self.u_data.items.get(projectee_key).get_r_axt(),
        );
        // Do nothing if ranges are equal
        if u_prange == old_u_prange {
            return Ok(());
        }
        let u_module = self.u_data.items.get_mut(item_key).get_module_mut().unwrap();
        let charge_key = u_module.get_charge_key();
        // Update user data for module
        u_module.get_projs_mut().add(projectee_key, u_prange);
        // Update user data for charge
        if let Some(charge_key) = charge_key {
            let u_charge = self.u_data.items.get_mut(charge_key).get_charge_mut().unwrap();
            u_charge.get_projs_mut().add(projectee_key, u_prange);
        }
        // Update services for module
        let u_item = self.u_data.items.get(item_key);
        let projectee_u_item = self.u_data.items.get(projectee_key);
        SolarSystem::util_change_item_proj_range(
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
            SolarSystem::util_change_item_proj_range(
                &self.u_data,
                &mut self.svc,
                charge_key,
                charge_u_item,
                projectee_key,
                projectee_u_item,
                u_prange,
            );
        }
        Ok(())
    }
}
