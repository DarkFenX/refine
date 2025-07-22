use crate::{
    ad,
    sol::SolarSystem,
    svc::Svc,
    uad::{Uad, UadEffectUpdates, UadItem, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_item_without_projs(
        uad: &Uad,
        svc: &mut Svc,
        item_key: UadItemKey,
        uad_item: &UadItem,
        eupdates: &UadEffectUpdates,
    ) {
        svc.notify_item_added(uad, item_key, uad_item);
        if uad_item.is_loaded() {
            svc.notify_item_loaded(uad, item_key, uad_item)
        }
        SolarSystem::util_internal_switch_item_state_without_projs(
            uad,
            svc,
            item_key,
            uad_item,
            ad::AState::Ghost,
            uad_item.get_a_state(),
            eupdates,
        );
    }
    // When removing even projectable item from solar system, we assume that projections are handled
    // separately, on the layers above this function. This is needed not to clean up on-item
    // projection container, which often makes item removal more expensive due to borrow checker
    // rules
    pub(in crate::sol::api) fn util_remove_item_without_projs(
        uad: &Uad,
        svc: &mut Svc,
        item_key: UadItemKey,
        uad_item: &UadItem,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        uad_item.stop_all_reffs(reuse_eupdates, &uad.src);
        SolarSystem::util_internal_switch_item_state_without_projs(
            uad,
            svc,
            item_key,
            uad_item,
            uad_item.get_a_state(),
            ad::AState::Ghost,
            reuse_eupdates,
        );
        if uad_item.is_loaded() {
            svc.notify_item_unloaded(uad, item_key, uad_item)
        }
        svc.notify_item_removed(uad, item_key, uad_item);
    }
    // "With projections" in this case means that projections will be handled when starting effects,
    // to emit effect projected/unprojected notifications. Notifications "projection added" is not
    // part of it
    pub(in crate::sol::api) fn util_add_item_with_projs(
        uad: &Uad,
        svc: &mut Svc,
        item_key: UadItemKey,
        uad_item: &UadItem,
        eupdates: &UadEffectUpdates,
    ) {
        svc.notify_item_added(uad, item_key, uad_item);
        if uad_item.is_loaded() {
            svc.notify_item_loaded(uad, item_key, uad_item);
        }
        SolarSystem::util_internal_switch_item_state(
            uad,
            svc,
            item_key,
            uad_item,
            ad::AState::Ghost,
            uad_item.get_a_state(),
            eupdates,
        );
    }
    // "With projections" in this case means that projections will be handled when stopping effects,
    // to emit effect projected/unprojected notifications. Notifications "projection added" is not
    // part of it
    pub(in crate::sol::api) fn util_remove_item_with_projs(
        uad: &Uad,
        svc: &mut Svc,
        item_key: UadItemKey,
        uad_item: &UadItem,
        reuse_eupdates: &mut UadEffectUpdates,
    ) {
        uad_item.stop_all_reffs(reuse_eupdates, &uad.src);
        SolarSystem::util_internal_switch_item_state(
            uad,
            svc,
            item_key,
            uad_item,
            uad_item.get_a_state(),
            ad::AState::Ghost,
            reuse_eupdates,
        );
        if uad_item.is_loaded() {
            svc.notify_item_unloaded(uad, item_key, uad_item)
        }
        svc.notify_item_removed(uad, item_key, uad_item);
    }
}
