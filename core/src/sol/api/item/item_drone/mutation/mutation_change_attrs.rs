use crate::{
    misc::AttrMutationRequest,
    sol::SolarSystem,
    uad::{UadItemKey, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_change_drone_mutation_attrs(
        &mut self,
        item_key: UadItemKey,
        attr_mutations: Vec<AttrMutationRequest>,
    ) -> Result<(), ItemMutatedError> {
        let uad_drone = self.uad.items.get_mut(item_key).get_drone_mut().unwrap();
        let changed_a_attr_ids = uad_drone.change_mutation_attrs(&self.uad.src, attr_mutations)?;
        for a_attr_id in changed_a_attr_ids {
            self.svc.notify_base_attr_value_changed(&self.uad, item_key, a_attr_id);
        }
        Ok(())
    }
}
