use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn util_add_charge(
        u_data: &mut UData,
        svc: &mut Svc,
        charge_key: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(charge_key);
        u_item.update_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_add_item(u_data, svc, charge_key, reuse_eupdates);
    }
    pub(in crate::api) fn util_remove_charge(
        u_data: &mut UData,
        svc: &mut Svc,
        charge_key: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(charge_key);
        u_item.stop_all_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_remove_item(u_data, svc, charge_key, reuse_eupdates);
    }
}
