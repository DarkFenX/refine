use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_module(
        u_data: &mut UData,
        svc: &mut Svc,
        module_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Option<UItemKey> {
        let u_module = u_data.items.get_mut(module_key).get_module_mut().unwrap();
        u_module.update_reffs(reuse_eupdates, &u_data.src);
        let charge_key = u_module.get_charge_key();
        SolarSystem::util_add_item(u_data, svc, module_key, reuse_eupdates);
        charge_key
    }
    pub(in crate::sol::api) fn util_add_module_with_charge_act(
        u_data: &mut UData,
        svc: &mut Svc,
        module_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Process module itself
        let charge_key = SolarSystem::util_add_module(u_data, svc, module_key, reuse_eupdates);
        // Process charge
        if let Some(charge_key) = charge_key
            && let Some(charge_activated) = reuse_eupdates.charge
        {
            SolarSystem::util_process_charge_activation(u_data, svc, charge_key, charge_activated, reuse_eupdates);
        }
    }
    pub(in crate::sol::api) fn util_remove_module(
        u_data: &mut UData,
        svc: &mut Svc,
        module_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Option<UItemKey> {
        let u_module = u_data.items.get_mut(module_key).get_module_mut().unwrap();
        u_module.stop_all_reffs(reuse_eupdates, &u_data.src);
        let charge_key = u_module.get_charge_key();
        SolarSystem::util_remove_item(u_data, svc, module_key, reuse_eupdates);
        charge_key
    }
    pub(in crate::sol::api) fn util_remove_module_with_charge_act(
        u_data: &mut UData,
        svc: &mut Svc,
        module_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Process module itself
        let charge_key = SolarSystem::util_remove_module(u_data, svc, module_key, reuse_eupdates);
        // Process charge
        if let Some(charge_key) = charge_key
            && let Some(charge_activated) = reuse_eupdates.charge
        {
            SolarSystem::util_process_charge_activation(u_data, svc, charge_key, charge_activated, reuse_eupdates);
        }
    }
}
