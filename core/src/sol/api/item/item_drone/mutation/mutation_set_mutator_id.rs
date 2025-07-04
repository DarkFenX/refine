use crate::{
    ad,
    def::{ItemKey, OF},
    sol::SolarSystem,
    uad::err::ItemMutatedError,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_a_mutator_id(
        &mut self,
        item_key: ItemKey,
        a_mutator_id: ad::AItemId,
    ) -> Result<(), ItemMutatedError> {
        let uad_item = self.uad.items.get(item_key);
        let mutation_data = match uad_item.get_mutation_data() {
            Some(mutation_data) => mutation_data,
            None => {
                return Err(ItemMutatedError {
                    item_id: self.uad.items.id_by_key(item_key),
                });
            }
        };
        if mutation_data.get_a_mutator_id() == a_mutator_id {
            return Ok(());
        }
        SolarSystem::util_remove_drone_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        // Set mutator ID and update UAD item
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        uad_drone.set_a_mutator_id(&self.uad.src, a_mutator_id).unwrap();
        // Update drone radius for outgoing projections
        let drone_radius = uad_drone.get_a_extras().and_then(|v| v.radius).unwrap_or(OF(0.0));
        for uad_prange in uad_drone.get_projs_mut().iter_ranges_mut() {
            uad_prange.update_src_rad(drone_radius);
        }
        // Update drone radius for incoming projections
        for &projector_item_key in self.rprojs.iter_projectors(&item_key) {
            let projector_uad_item = self.uad.items.get_mut(projector_item_key);
            if let Some(uad_prange) = projector_uad_item.get_projs_mut().unwrap().get_mut_range(&item_key) {
                if uad_prange.update_tgt_rad(drone_radius) {
                    let uad_prange = Some(*uad_prange);
                    let uad_item = self.uad.items.get(item_key);
                    SolarSystem::util_change_item_proj_range(
                        &self.uad,
                        &mut self.svc,
                        &self.reffs,
                        projector_item_key,
                        item_key,
                        uad_item,
                        uad_prange,
                    );
                }
            }
        }
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_add_drone_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        Ok(())
    }
}
