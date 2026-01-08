use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn util_add_drone(
        u_data: &mut UData,
        svc: &mut Svc,
        drone_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(drone_uid);
        u_item.update_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_add_item(u_data, svc, drone_uid, reuse_eupdates);
    }
    pub(in crate::api) fn util_remove_drone(
        u_data: &mut UData,
        svc: &mut Svc,
        drone_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(drone_uid);
        u_item.stop_all_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_remove_item(u_data, svc, drone_uid, reuse_eupdates);
    }
}
