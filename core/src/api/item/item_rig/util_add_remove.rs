use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn util_add_rig(
        u_data: &mut UData,
        svc: &mut Svc,
        rig_key: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(rig_key);
        u_item.update_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_add_item(u_data, svc, rig_key, reuse_eupdates);
    }
    pub(in crate::api) fn util_remove_rig(
        u_data: &mut UData,
        svc: &mut Svc,
        rig_key: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(rig_key);
        u_item.stop_all_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_remove_item(u_data, svc, rig_key, reuse_eupdates);
    }
}
