use crate::{
    misc::AttrMutationRequest,
    sol::SolarSystem,
    ud::{UItemKey, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_change_drone_mutation_attrs(
        &mut self,
        drone_key: UItemKey,
        attr_mutations: Vec<AttrMutationRequest>,
    ) -> Result<(), ItemMutatedError> {
        let u_drone = self.u_data.items.get_mut(drone_key).dc_drone_mut().unwrap();
        let changed_attr_keys = u_drone.change_mutation_attrs(&self.u_data.src, attr_mutations)?;
        for attr_key in changed_attr_keys {
            self.svc
                .notify_base_attr_value_changed(&self.u_data, drone_key, attr_key);
        }
        Ok(())
    }
}
