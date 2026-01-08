use crate::{
    rd::RState,
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn util_switch_item_state(
        u_data: &UData,
        svc: &mut Svc,
        item_uid: UItemId,
        old_state: RState,
        new_state: RState,
        eupdates: &UEffectUpdates,
    ) {
        if new_state != old_state {
            let u_item = u_data.items.get(item_uid);
            SolarSystem::util_internal_switch_item_state(u_data, svc, item_uid, u_item, old_state, new_state, eupdates);
        }
    }
    pub(in crate::api::util) fn util_internal_switch_item_state(
        u_data: &UData,
        svc: &mut Svc,
        item_uid: UItemId,
        u_item: &UItem,
        old_state: RState,
        new_state: RState,
        eupdates: &UEffectUpdates,
    ) {
        switch_item_state(svc, item_uid, u_item, old_state, new_state);
        SolarSystem::util_internal_process_effect_updates(u_data, svc, item_uid, u_item, eupdates);
    }
}

fn switch_item_state(svc: &mut Svc, item_uid: UItemId, u_item: &UItem, old_state: RState, new_state: RState) {
    match new_state.cmp(&old_state) {
        std::cmp::Ordering::Equal => (),
        std::cmp::Ordering::Greater => {
            let is_item_loaded = u_item.is_loaded();
            for state in RState::iter().filter(|v| *v > old_state && *v <= new_state) {
                svc.notify_state_activated(item_uid, u_item, state);
                if is_item_loaded {
                    svc.notify_item_state_activated_loaded(item_uid, u_item, state);
                }
            }
        }
        std::cmp::Ordering::Less => {
            let is_item_loaded = u_item.is_loaded();
            for state in RState::iter().rev().filter(|v| *v > new_state && *v <= old_state) {
                if is_item_loaded {
                    svc.notify_item_state_deactivated_loaded(&item_uid, u_item, state);
                }
                svc.notify_state_deactivated(&item_uid, u_item, state);
            }
        }
    }
}
