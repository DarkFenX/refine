use crate::{
    misc::AttrMutationRequest,
    sol::SolarSystem,
    ud::{UItemKey, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_change_module_mutation_attrs(
        &mut self,
        module_key: UItemKey,
        attr_mutations: Vec<AttrMutationRequest>,
    ) -> Result<(), ItemMutatedError> {
        let u_module = self.u_data.items.get_mut(module_key).get_module_mut().unwrap();
        let changed_a_attr_ids = u_module.change_mutation_attrs(&self.u_data.src, attr_mutations)?;
        for a_attr_id in changed_a_attr_ids {
            self.svc
                .notify_base_attr_value_changed(&self.u_data, module_key, a_attr_id);
        }
        Ok(())
    }
}
