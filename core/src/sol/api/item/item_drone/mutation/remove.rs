use crate::sol::{ItemKey, SolarSystem, err::ItemMutatedError};

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
        self.svc.unload_item(&self.uad, item_key, uad_item);
        self.uad
            .items
            .get_mut(item_key)
            .get_drone_mut()
            .unwrap()
            .unmutate(&self.uad.src)
            .unwrap();
        let uad_item = self.uad.items.get(item_key);
        self.svc.load_item(&self.uad, item_key, uad_item);
        Ok(())
    }
}
