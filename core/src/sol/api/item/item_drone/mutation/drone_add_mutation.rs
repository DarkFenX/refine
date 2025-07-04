use crate::{
    def::{ItemKey, ItemTypeId, OF},
    err::basic::ItemNotMutatedError,
    misc::ItemMutationRequest,
    sol::{
        SolarSystem,
        api::{AddMutationError, DroneMut, MutationMut},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_add_drone_mutation(
        &mut self,
        item_key: ItemKey,
        mutation: ItemMutationRequest,
    ) -> Result<(), ItemNotMutatedError> {
        let uad_item = self.uad.items.get(item_key);
        SolarSystem::util_remove_drone_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
        // Process mutation request and update UAD item
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        if let Err(error) = uad_drone.mutate(&self.uad.src, mutation) {
            let uad_item = self.uad.items.get(item_key);
            SolarSystem::util_add_drone_with_projs(&self.uad, &mut self.svc, &mut self.reffs, item_key, uad_item);
            return Err(error);
        }
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

impl<'a> DroneMut<'a> {
    pub fn mutate(&mut self, mutator_id: ItemTypeId) -> Result<MutationMut<'_>, AddMutationError> {
        let mutation = ItemMutationRequest {
            mutator_id,
            attrs: Vec::new(),
        };
        self.sol.internal_add_drone_mutation(self.key, mutation)?;
        Ok(self.get_mutation_mut().unwrap())
    }
}
