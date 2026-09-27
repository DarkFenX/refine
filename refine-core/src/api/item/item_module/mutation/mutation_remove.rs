use crate::{
    sol::SolarSystem,
    ud::{UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api::item) fn internal_remove_module_mutation(
        &mut self,
        module_uid: UItemId,
    ) -> Result<(), ItemMutatedError> {
        SolarSystem::util_remove_module_with_charge_act(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            &mut self.cache.eupdates,
        );
        let u_module = self.u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        if let Err(error) = u_module.unmutate(&self.u_data.r_data) {
            SolarSystem::util_add_module_with_charge_act(
                &mut self.u_data,
                &mut self.svc,
                module_uid,
                &mut self.cache.eupdates,
            );
            return Err(error);
        }
        SolarSystem::util_add_module_with_charge_act(
            &mut self.u_data,
            &mut self.svc,
            module_uid,
            &mut self.cache.eupdates,
        );
        Ok(())
    }
}
