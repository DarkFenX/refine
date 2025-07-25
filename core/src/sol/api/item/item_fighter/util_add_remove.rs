use crate::{
    sol::{SolarSystem, rev_projs::RevProjs},
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_fighter_with_projs(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        item_key: UItemKey,
        reuse_eupdates: &UEffectUpdates,
    ) {
        // Process fighter itself
        let u_item = u_data.items.get(item_key);
        SolarSystem::util_add_item_with_projs(u_data, svc, item_key, u_item, reuse_eupdates);
        // Process autocharges
        SolarSystem::add_fighter_autocharges(u_data, svc, rev_projs, item_key);
    }
    pub(in crate::sol::api) fn util_remove_fighter_with_projs(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Process autocharges
        SolarSystem::remove_fighter_autocharges(u_data, svc, rev_projs, item_key, true, reuse_eupdates);
        // Process fighter itself
        let u_item = u_data.items.get(item_key);
        SolarSystem::util_remove_item_with_projs(u_data, svc, item_key, u_item, reuse_eupdates);
    }
}
