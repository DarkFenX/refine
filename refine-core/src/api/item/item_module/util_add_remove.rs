use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(super) fn util_add_module(
        u_data: &mut UData,
        svc: &mut Svc,
        module_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Option<UItemId> {
        let u_module = u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.update_reffs(reuse_eupdates, &u_data.r_data);
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
    pub(in crate::api) fn util_add_module_with_charge(
        u_data: &mut UData,
        svc: &mut Svc,
        module_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Process module itself
        let charge_uid = SolarSystem::util_add_module(u_data, svc, module_uid, reuse_eupdates);
        // Process charge - its activation is defined by module effects which have just been started
        if let Some(charge_uid) = charge_uid {
            let u_charge = u_data.items.get_mut(charge_uid).dc_charge_mut().unwrap();
            u_charge.set_activated(reuse_eupdates.charge.unwrap_or(false));
            SolarSystem::util_add_charge(u_data, svc, charge_uid, reuse_eupdates);
        }
    }
    pub(super) fn util_remove_module(
        u_data: &mut UData,
        svc: &mut Svc,
        module_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Option<UItemId> {
        let u_module = u_data.items.get_mut(module_uid).dc_module_mut().unwrap();
        u_module.stop_all_reffs(reuse_eupdates, &u_data.r_data);
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
    pub(in crate::api) fn util_remove_module_with_charge(
        u_data: &mut UData,
        svc: &mut Svc,
        module_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Process charge - it is removed in its current state, so that removing module does not
        // need to touch it
        let charge_uid = u_data.items.get(module_uid).dc_module().unwrap().get_charge_uid();
        if let Some(charge_uid) = charge_uid {
            SolarSystem::util_remove_charge(u_data, svc, charge_uid, reuse_eupdates);
        }
        // Process module itself
        SolarSystem::util_remove_module(u_data, svc, module_uid, reuse_eupdates);
    }
}
