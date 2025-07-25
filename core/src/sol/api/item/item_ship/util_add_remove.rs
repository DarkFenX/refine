use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_ship(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        reuse_eupdates: &UEffectUpdates,
    ) {
        // TODO: consider moving fit kind update here
        let u_item = u_data.items.get(item_key);
        SolarSystem::util_add_item_without_projs(u_data, svc, item_key, u_item, reuse_eupdates);
    }
    pub(in crate::sol::api) fn util_remove_ship(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        u_item: &UItem,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // TODO: consider moving fit kind update here
        SolarSystem::util_remove_item_without_projs(u_data, svc, item_key, u_item, reuse_eupdates);
    }
}
