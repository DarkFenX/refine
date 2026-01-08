use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn util_add_module(
        u_data: &mut UData,
        svc: &mut Svc,
        module_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Option<UItemId> {
        let u_module = u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.update_reffs(reuse_eupdates, &u_data.src);
        let charge_uid = u_module.get_charge_uid();
        SolarSystem::util_add_item(u_data, svc, module_uid, reuse_eupdates);
        charge_uid
    }
    pub(in crate::api) fn util_add_module_with_charge_act(
        u_data: &mut UData,
        svc: &mut Svc,
        module_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Process module itself
        let charge_uid = SolarSystem::util_add_module(u_data, svc, module_uid, reuse_eupdates);
        // Process charge
        if let Some(charge_uid) = charge_uid
            && let Some(charge_activated) = reuse_eupdates.charge
        {
            SolarSystem::util_process_charge_activation(u_data, svc, charge_uid, charge_activated, reuse_eupdates);
        }
    }
    pub(in crate::api) fn util_remove_module(
        u_data: &mut UData,
        svc: &mut Svc,
        module_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Option<UItemId> {
        let u_module = u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.stop_all_reffs(reuse_eupdates, &u_data.src);
        let charge_uid = u_module.get_charge_uid();
        SolarSystem::util_remove_item(u_data, svc, module_uid, reuse_eupdates);
        charge_uid
    }
    pub(in crate::api) fn util_remove_module_with_charge_act(
        u_data: &mut UData,
        svc: &mut Svc,
        module_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Process module itself
        let charge_uid = SolarSystem::util_remove_module(u_data, svc, module_uid, reuse_eupdates);
        // Process charge
        if let Some(charge_uid) = charge_uid
            && let Some(charge_activated) = reuse_eupdates.charge
        {
            SolarSystem::util_process_charge_activation(u_data, svc, charge_uid, charge_activated, reuse_eupdates);
        }
    }
}
