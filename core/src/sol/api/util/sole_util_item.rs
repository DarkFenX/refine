use crate::{
    ad::AState,
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_item(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        eupdates: &UEffectUpdates,
    ) {
        let u_item = u_data.items.get(item_key);
        svc.notify_item_added(u_data, item_key, u_item);
        if u_item.is_loaded() {
            svc.notify_item_loaded(u_data, item_key, u_item);
        }
        SolarSystem::util_internal_switch_item_state(
            u_data,
            svc,
            item_key,
            u_item,
            AState::Ghost,
            u_item.get_state(),
            eupdates,
        );
    }
    // "With projections" in this case means that projections will be handled when stopping effects,
    // to emit effect projected/unprojected notifications. Notifications "projection added" is not
    // part of it
    pub(in crate::sol::api) fn util_remove_item(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get(item_key);
        SolarSystem::util_internal_switch_item_state(
            u_data,
            svc,
            item_key,
            u_item,
            u_item.get_state(),
            AState::Ghost,
            reuse_eupdates,
        );
        if u_item.is_loaded() {
            svc.notify_item_unloaded(u_data, item_key, u_item)
        }
        svc.notify_item_removed(u_data, item_key, u_item);
    }
}
