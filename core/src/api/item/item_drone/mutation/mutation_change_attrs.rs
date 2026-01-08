use crate::{
    sol::SolarSystem,
    ud::{UAttrMutationRequest, UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_change_drone_mutation_attrs(
        &mut self,
        drone_uid: UItemId,
        attr_mutations: Vec<UAttrMutationRequest>,
    ) -> Result<(), ItemMutatedError> {
        let u_drone = self.u_data.items.get_mut(drone_uid).dc_drone_mut().unwrap();
        let changed_attr_rids = u_drone.change_mutation_attrs(&self.u_data.src, attr_mutations)?;
        for attr_rid in changed_attr_rids {
            self.svc
                .notify_base_attr_value_changed(&self.u_data, drone_uid, attr_rid);
        }
        Ok(())
    }
}
