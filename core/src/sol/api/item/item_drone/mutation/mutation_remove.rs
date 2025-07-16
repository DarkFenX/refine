use crate::{
    def::ItemKey,
    sol::SolarSystem,
    uad::{UadEffectUpdates, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_drone_mutation(
        &mut self,
        item_key: ItemKey,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> Result<(), ItemMutatedError> {
        let uad_item = self.uad.items.get(item_key);
        let uad_drone = uad_item.get_drone().unwrap();
        if uad_drone.get_mutation_data().is_none() {
            return Err(ItemMutatedError {
                item_id: uad_drone.get_item_id(),
            });
        }
        SolarSystem::util_remove_drone_with_projs(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        self.uad
            .items
            .get_mut(item_key)
            .get_drone_mut()
            .unwrap()
            .unmutate(reuse_eupdates, &self.uad.src)
            .unwrap();
        SolarSystem::util_update_item_radius_in_projs(&mut self.uad, &self.rprojs, &mut self.svc, item_key);
        SolarSystem::util_add_drone_with_projs(&self.uad, &mut self.svc, item_key, reuse_eupdates);
        Ok(())
    }
}
