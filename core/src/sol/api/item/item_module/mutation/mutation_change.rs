use crate::{def::ItemKey, misc::AttrMutationRequest, sol::SolarSystem, uad::err::ItemMutatedError};

impl SolarSystem {
    pub(in crate::sol) fn internal_change_module_mutation_attrs(
        &mut self,
        item_key: ItemKey,
        attr_mutations: Vec<AttrMutationRequest>,
    ) -> Result<(), ItemMutatedError> {
        let uad_module = self.uad.items.get_mut(item_key).get_module_mut().unwrap();
        let changed_a_attr_ids = uad_module.change_mutation_attrs(&self.uad.src, attr_mutations)?;
        for a_attr_id in changed_a_attr_ids {
            self.svc.notify_base_attr_value_changed(&self.uad, item_key, a_attr_id);
        }
        Ok(())
    }
}
