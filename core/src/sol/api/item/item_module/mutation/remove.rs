use crate::{
    err::basic::ItemMutatedError,
    sol::{ItemKey, SolarSystem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_module_mutation(
        &mut self,
        item_key: ItemKey,
    ) -> Result<(), ItemMutatedError> {
        let uad_item = self.uad.items.get(item_key);
        let uad_module = uad_item.get_module().unwrap();
        if uad_module.get_mutation_data().is_none() {
            return Err(ItemMutatedError {
                item_id: uad_module.get_item_id(),
            });
        }
        self.svc.unload_item(&self.uad, item_key, uad_item);
        self.uad
            .items
            .get_mut(item_key)
            .get_module_mut()
            .unwrap()
            .unmutate(&self.uad.src)
            .unwrap();
        let uad_item = self.uad.items.get(item_key);
        self.svc.load_item(&self.uad, item_key, uad_item);
        Ok(())
    }
}
