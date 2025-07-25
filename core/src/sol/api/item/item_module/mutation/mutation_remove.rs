use crate::{
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_module_mutation(
        &mut self,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemMutatedError> {
        let u_item = self.u_data.items.get(item_key);
        let u_module = u_item.get_module().unwrap();
        if u_module.get_mutation_data().is_none() {
            return Err(ItemMutatedError {
                item_id: u_module.get_item_id(),
            });
        }
        SolarSystem::util_remove_module_with_projs(&self.u_data, &mut self.svc, item_key, u_item, reuse_eupdates);
        self.u_data
            .items
            .get_mut(item_key)
            .get_module_mut()
            .unwrap()
            .unmutate(reuse_eupdates, &self.u_data.src)
            .unwrap();
        SolarSystem::util_add_module_with_projs(&self.u_data, &mut self.svc, item_key, reuse_eupdates);
        Ok(())
    }
}
