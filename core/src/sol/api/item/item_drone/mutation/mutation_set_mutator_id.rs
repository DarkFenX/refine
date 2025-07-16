use crate::{
    ad,
    def::ItemKey,
    sol::SolarSystem,
    uad::{UadEffectUpdates, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_a_mutator_id(
        &mut self,
        item_key: ItemKey,
        a_mutator_id: ad::AItemId,
        reuse_eupdates: &mut UadEffectUpdates,
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
        SolarSystem::util_remove_drone_with_projs(&self.uad, &mut self.svc, item_key, uad_item, reuse_eupdates);
        self.uad
            .items
            .get_mut(item_key)
            .get_drone_mut()
            .unwrap()
            .set_a_mutator_id(a_mutator_id, reuse_eupdates, &self.uad.src)
            .unwrap();
        SolarSystem::util_update_item_radius_in_projs(&mut self.uad, &self.rprojs, &mut self.svc, item_key);
        SolarSystem::util_add_drone_with_projs(&self.uad, &mut self.svc, item_key, reuse_eupdates);
        Ok(())
    }
}
