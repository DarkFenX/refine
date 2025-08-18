use crate::{
    sol::{SolarSystem, rev_projs::RevProjs},
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_fighter(
        u_data: &mut UData,
        svc: &mut Svc,
        fighter_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(fighter_key);
        u_item.update_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_add_item(u_data, svc, fighter_key, reuse_eupdates);
    }
    pub(in crate::sol::api) fn util_add_fighter_with_acs(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        fighter_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Process fighter itself
        SolarSystem::util_add_fighter(u_data, svc, fighter_key, reuse_eupdates);
        // Process autocharges
        SolarSystem::add_item_autocharges(u_data, svc, rev_projs, fighter_key, reuse_eupdates);
    }
    pub(in crate::sol::api) fn util_remove_fighter(
        u_data: &mut UData,
        svc: &mut Svc,
        fighter_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(fighter_key);
        u_item.stop_all_reffs(reuse_eupdates, &u_data.src);
        SolarSystem::util_remove_item(u_data, svc, fighter_key, reuse_eupdates);
    }
    pub(in crate::sol::api) fn util_remove_fighter_with_acs(
        u_data: &mut UData,
        svc: &mut Svc,
        rev_projs: &mut RevProjs,
        fighter_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        // Process autocharges
        SolarSystem::remove_item_autocharges(u_data, svc, rev_projs, fighter_key, reuse_eupdates);
        // Process fighter itself
        SolarSystem::util_remove_fighter(u_data, svc, fighter_key, reuse_eupdates);
    }
}
