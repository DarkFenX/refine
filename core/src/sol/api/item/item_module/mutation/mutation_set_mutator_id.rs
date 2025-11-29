use crate::{
    ad::AItemId,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItemKey, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_module_a_mutator_id(
        &mut self,
        module_key: UItemKey,
        mutator_id: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemMutatedError> {
        SolarSystem::util_remove_module_with_charge_act(&mut self.u_data, &mut self.svc, module_key, reuse_eupdates);
        let u_module = self.u_data.items.get_mut(module_key).dc_module_mut().unwrap();
        if let Err(error) = u_module.set_mutator_id(mutator_id, &self.u_data.src) {
            SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, module_key, reuse_eupdates);
            return Err(error);
        }
        SolarSystem::util_add_module_with_charge_act(&mut self.u_data, &mut self.svc, module_key, reuse_eupdates);
        Ok(())
    }
}
