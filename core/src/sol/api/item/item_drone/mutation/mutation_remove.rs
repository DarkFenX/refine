use crate::{
    def::{ItemKey, OF},
    sol::SolarSystem,
    uad::err::ItemMutatedError,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_drone_mutation(
        &mut self,
        item_key: ItemKey,
    ) -> Result<(), ItemMutatedError> {
        let uad_item = self.uad.items.get(item_key);
        let uad_drone = uad_item.get_drone().unwrap();
        if uad_drone.get_mutation_data().is_none() {
            return Err(ItemMutatedError {
                item_id: uad_drone.get_item_id(),
            });
        }
        SolarSystem::util_remove_drone_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        // Remove mutation and update UAD item
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        uad_drone.unmutate(&self.uad.src).unwrap();
        // Update drone radius for outgoing projections
        let drone_radius = uad_drone.get_a_extras().and_then(|v| v.radius).unwrap_or(OF(0.0));
        for uad_prange in uad_drone.get_projs_mut().iter_ranges_mut() {
            uad_prange.update_src_rad(drone_radius);
        }
        // Update drone radius for incoming projections
        let mut updated_projectors = Vec::new();
        for &projector_item_key in self.rprojs.iter_projectors(&item_key) {
            let projector_uad_item = self.uad.items.get_mut(projector_item_key);
            if let Some(uad_prange) = projector_uad_item.get_projs_mut().unwrap().get_mut_range(&item_key) {
                if uad_prange.update_tgt_rad(drone_radius) {
                    updated_projectors.push((projector_item_key, *uad_prange));
                }
            }
        }
        let uad_item = self.uad.items.get(item_key);
        for (projector_item_key, uad_prange) in updated_projectors {
            SolarSystem::util_change_item_proj_range(
                &self.uad,
                &mut self.svc,
                &self.reffs,
                projector_item_key,
                item_key,
                uad_item,
                Some(uad_prange),
            );
        }
        SolarSystem::util_add_drone_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        Ok(())
    }
}
