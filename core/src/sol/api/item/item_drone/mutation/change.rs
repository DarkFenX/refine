use crate::sol::{ItemChangeAttrMutation, ItemKey, SolarSystem, err::ItemMutatedError};

impl SolarSystem {
    pub(in crate::sol) fn internal_change_drone_mutation(
        &mut self,
        item_key: ItemKey,
        attr_mutations: Vec<ItemChangeAttrMutation>,
    ) -> Result<(), ItemMutatedError> {
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        let changed_a_attr_ids = uad_drone.change_mutation_attrs(&self.uad.src, attr_mutations)?;
        for a_attr_id in changed_a_attr_ids {
            self.svc.item_base_attr_value_changed(&self.uad, item_key, a_attr_id);
        }
        Ok(())
    }
}
