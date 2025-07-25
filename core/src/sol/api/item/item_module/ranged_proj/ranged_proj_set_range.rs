use crate::{
    err::basic::ProjFoundError,
    misc::ProjRange,
    sol::{SolarSystem, api::get_r_ship_axt},
    uad::{UadItemKey, UadProjRange},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_proj_range(
        &mut self,
        item_key: UadItemKey,
        projectee_key: UadItemKey,
        range: ProjRange,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined before changing it
        let uad_module = self.uad.items.get(item_key).get_module().unwrap();
        let old_uad_prange = uad_module
            .get_projs()
            .get(&projectee_key)
            .ok_or_else(|| ProjFoundError {
                projector_item_id: uad_module.get_item_id(),
                projectee_item_id: self.uad.items.id_by_key(projectee_key),
            })?;
        let uad_prange = UadProjRange::from_prange_with_axt(
            range,
            get_r_ship_axt(&self.uad, uad_module.get_fit_key()),
            self.uad.items.get(projectee_key).get_r_axt(),
        );
        // Do nothing if ranges are equal
        if uad_prange == old_uad_prange {
            return Ok(());
        }
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        let charge_key = uad_module.get_charge_key();
        // Update user data for module
        uad_module.get_projs_mut().add(projectee_key, uad_prange);
        // Update user data for charge
        if let Some(charge_key) = charge_key {
            let uad_charge = self.uad.items.get_mut(charge_key).get_charge_mut().unwrap();
            uad_charge.get_projs_mut().add(projectee_key, uad_prange);
        }
        // Update services for module
        let uad_item = self.uad.items.get(item_key);
        let projectee_uad_item = self.uad.items.get(projectee_key);
        SolarSystem::util_change_item_proj_range(
            &self.uad,
            &mut self.svc,
            item_key,
            uad_item,
            projectee_key,
            projectee_uad_item,
            uad_prange,
        );
        // Update services for charge
        if let Some(charge_key) = charge_key {
            let charge_uad_item = self.uad.items.get(charge_key);
            SolarSystem::util_change_item_proj_range(
                &self.uad,
                &mut self.svc,
                charge_key,
                charge_uad_item,
                projectee_key,
                projectee_uad_item,
                uad_prange,
            );
        }
        Ok(())
    }
}
