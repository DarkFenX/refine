use crate::{
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_module_mutation(
        &mut self,
        module_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemMutatedError> {
        SolarSystem::util_remove_module_with_charge_act(&mut self.u_data, &mut self.svc, module_key, reuse_eupdates);
        let u_module = self.u_data.items.get_mut(module_key).dc_module_mut().unwrap();
        if let Err(error) = u_module.unmutate(&self.u_data.src) {
            SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, module_key, reuse_eupdates);
            return Err(error);
        }
        SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, module_key, reuse_eupdates);
        Ok(())
    }
}
