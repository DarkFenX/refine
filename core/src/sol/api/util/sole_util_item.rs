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
    pub(in crate::sol::api) fn util_add_item(
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
        // When adding item, we assume it has no projections - they should be added to services by
        // the caller after calling this function
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
    pub(in crate::sol::api) fn util_remove_item(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        item_key: ItemKey,
        uad_item: &UadItem,
    ) {
        // When removing item, we assume that projections are handled separately. This is needed not
        // to clean up on-item projection container, which often makes item removal more expensive
        // due to borrow checker rules
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
    pub(in crate::sol::api) fn util_load_item(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        item_key: ItemKey,
        uad_item: &UadItem,
    ) {
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
    pub(in crate::sol::api) fn util_unload_item(
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
    }
}
