use crate::{
    ad,
    sol::{
        ItemKey, SolarSystem,
        reffs::REffs,
        svc::Svc,
        uad::{Uad, item::UadItem},
    },
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_add_item_without_projs(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        item_key: ItemKey,
        uad_item: &UadItem,
    ) {
        svc.notify_item_added(uad, item_key, uad_item);
        if uad_item.is_loaded() {
            svc.notify_item_loaded(uad, item_key, uad_item)
        }
        SolarSystem::util_internal_switch_item_state_without_projs(
            uad,
            svc,
            reffs,
            item_key,
            uad_item,
            ad::AState::Ghost,
            uad_item.get_a_state(),
        );
    }
    // When removing even projectable item from solar system, we assume that projections are handled
    // separately, on the layers above this function. This is needed not to clean up on-item
    // projection container, which often makes item removal more expensive due to borrow checker
    // rules
    pub(in crate::sol::api) fn util_remove_item_without_projs(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        item_key: ItemKey,
        uad_item: &UadItem,
    ) {
        SolarSystem::util_internal_switch_item_state_without_projs(
            uad,
            svc,
            reffs,
            item_key,
            uad_item,
            uad_item.get_a_state(),
            ad::AState::Ghost,
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
        reffs: &mut REffs,
        item_key: ItemKey,
        uad_item: &UadItem,
    ) {
        svc.notify_item_added(uad, item_key, uad_item);
        if uad_item.is_loaded() {
            svc.notify_item_loaded(uad, item_key, uad_item);
        }
        SolarSystem::util_internal_switch_item_state(
            uad,
            svc,
            reffs,
            item_key,
            uad_item,
            ad::AState::Ghost,
            uad_item.get_a_state(),
        );
    }
    // "With projections" in this case means that projections will be handled when stopping effects,
    // to emit effect projected/unprojected notifications. Notifications "projection added" is not
    // part of it
    pub(in crate::sol::api) fn util_remove_item_with_projs(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        item_key: ItemKey,
        uad_item: &UadItem,
    ) {
        SolarSystem::util_internal_switch_item_state(
            uad,
            svc,
            reffs,
            item_key,
            uad_item,
            uad_item.get_a_state(),
            ad::AState::Ghost,
        );
        if uad_item.is_loaded() {
            svc.notify_item_unloaded(uad, item_key, uad_item)
        }
        svc.notify_item_removed(uad, item_key, uad_item);
    }
}
