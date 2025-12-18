use crate::{
    misc::AttrMutationRequest,
    sol::SolarSystem,
    ud::{UItemKey, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_change_module_mutation_attrs(
        &mut self,
        module_key: UItemKey,
        attr_mutations: Vec<AttrMutationRequest>,
    ) -> Result<(), ItemMutatedError> {
        let u_module = self.u_data.items.get_mut(module_key).dc_module_mut().unwrap();
        let changed_attr_keys = u_module.change_mutation_attrs(&self.u_data.src, attr_mutations)?;
        for attr_keys in changed_attr_keys {
            self.svc
                .notify_base_attr_value_changed(&self.u_data, module_key, attr_keys);
        }
        Ok(())
    }
}
