use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_ship(
        u_data: &mut UData,
        svc: &mut Svc,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // TODO: consider moving fit kind update here
        let u_item = u_data.items.get_mut(item_key);
        u_item.update_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_add_item_without_projs(u_data, svc, item_key, reuse_eupdates);
    }
    pub(in crate::sol::api) fn util_remove_ship(
        u_data: &mut UData,
        svc: &mut Svc,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // TODO: consider moving fit kind update here
        let u_item = u_data.items.get_mut(item_key);
        u_item.stop_all_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_remove_item_without_projs(u_data, svc, item_key, reuse_eupdates);
    }
}
