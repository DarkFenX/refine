use crate::{
    def::{AttrVal, ItemKey},
    err::basic::ProjFoundError,
    sol::SolarSystem,
    uad::ProjRange,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_proj_range(
        &mut self,
        item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) -> Result<(), ProjFoundError> {
        // Check if projection is defined before changing it
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        let old_range = match uad_module.get_projs().get(&projectee_item_key) {
            Some(old_range) => *old_range,
            None => {
                return Err(ProjFoundError {
                    projector_item_id: uad_module.get_item_id(),
                    projectee_item_id: self.uad.items.id_by_key(projectee_item_key),
                });
            }
        };
        // Do nothing if ranges are equal
        if range == old_range.map(|v| v.c2c) {
            return Ok(());
        }
        let charge_key = uad_module.get_charge_item_key();
        // Update user data for module
        uad_module
            .get_projs_mut()
            .add(projectee_item_key, range.map(ProjRange::new_tmp));
        // Update user data for charge
        if let Some(charge_key) = charge_key {
            let uad_charge = self.uad.items.get_mut(charge_key).get_charge_mut().unwrap();
            uad_charge
                .get_projs_mut()
                .add(projectee_item_key, range.map(ProjRange::new_tmp));
        }
        // Update services for module
        let projectee_uad_item = self.uad.items.get(projectee_item_key);
        SolarSystem::util_change_item_proj_range(
            &self.uad,
            &mut self.svc,
            &self.reffs,
            item_key,
            projectee_item_key,
            projectee_uad_item,
            range.map(ProjRange::new_tmp),
        );
        // Update services for charge
        if let Some(charge_key) = charge_key {
            SolarSystem::util_change_item_proj_range(
                &self.uad,
                &mut self.svc,
                &self.reffs,
                charge_key,
                projectee_item_key,
                projectee_uad_item,
                range.map(ProjRange::new_tmp),
            );
        }
        Ok(())
    }
}
