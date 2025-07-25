use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_drone_with_projs(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        reuse_eupdates: &UEffectUpdates,
    ) {
        let u_item = u_data.items.get(item_key);
        SolarSystem::util_add_item_with_projs(u_data, svc, item_key, u_item, reuse_eupdates);
    }
    pub(in crate::sol::api) fn util_remove_drone_with_projs(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        u_item: &UItem,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        SolarSystem::util_remove_item_with_projs(u_data, svc, item_key, u_item, reuse_eupdates);
    }
}
