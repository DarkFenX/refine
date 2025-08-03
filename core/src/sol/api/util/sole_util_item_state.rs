use crate::{
    ad,
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_switch_item_state(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        old_item_a_state: ad::AState,
        new_item_a_state: ad::AState,
        eupdates: &UEffectUpdates,
    ) {
        if new_item_a_state != old_item_a_state {
            let u_item = u_data.items.get(item_key);
            SolarSystem::util_internal_switch_item_state(
                u_data,
                svc,
                item_key,
                u_item,
                old_item_a_state,
                new_item_a_state,
                eupdates,
            );
        }
    }
    pub(in crate::sol::api::util) fn util_internal_switch_item_state(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        u_item: &UItem,
        old_item_a_state: ad::AState,
        new_item_a_state: ad::AState,
        eupdates: &UEffectUpdates,
    ) {
        switch_item_state(svc, item_key, u_item, old_item_a_state, new_item_a_state);
        SolarSystem::util_internal_process_effect_updates(u_data, svc, item_key, u_item, eupdates);
    }
}

fn switch_item_state(
    svc: &mut Svc,
    item_key: UItemKey,
    u_item: &UItem,
    old_item_a_state: ad::AState,
    new_item_a_state: ad::AState,
) {
    match new_item_a_state.cmp(&old_item_a_state) {
        std::cmp::Ordering::Equal => (),
        std::cmp::Ordering::Greater => {
            let is_item_loaded = u_item.is_loaded();
            for a_state in ad::AState::iter().filter(|v| **v > old_item_a_state && **v <= new_item_a_state) {
                svc.notify_state_activated(item_key, u_item, a_state);
                if is_item_loaded {
                    svc.notify_item_state_activated_loaded(item_key, u_item, a_state);
                }
            }
        }
        std::cmp::Ordering::Less => {
            let is_item_loaded = u_item.is_loaded();
            for a_state in ad::AState::iter()
                .rev()
                .filter(|v| **v > new_item_a_state && **v <= old_item_a_state)
            {
                if is_item_loaded {
                    svc.notify_item_state_deactivated_loaded(&item_key, u_item, a_state);
                }
                svc.notify_state_deactivated(&item_key, u_item, a_state);
            }
        }
    }
}
