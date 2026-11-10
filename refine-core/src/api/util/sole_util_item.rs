use crate::{
    rd::RState,
    sol::SolCore,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemId},
};

impl SolCore {
    pub(in crate::api) fn util_add_item(u_data: &UData, svc: &mut Svc, item_uid: UItemId, eupdates: &UEffectUpdates) {
        let u_item = u_data.items.get(item_uid);
        svc.notify_item_added(u_data, item_uid, u_item);
        if u_item.is_loaded() {
            svc.notify_item_loaded(u_data, item_uid, u_item);
        }
        SolCore::util_internal_switch_item_state(
            u_data,
            svc,
            item_uid,
            u_item,
            RState::Ghost,
            u_item.get_state(),
            eupdates,
        );
        // Items are added in ghost state and go up from there, so effect updates have nothing
        // stopped; not calling the effect stop handler because of it
    }
    pub(in crate::api) fn util_remove_item(
        u_data: &mut UData,
        svc: &mut Svc,
        item_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get(item_uid);
        SolCore::util_internal_switch_item_state(
            u_data,
            svc,
            item_uid,
            u_item,
            u_item.get_state(),
            RState::Ghost,
            reuse_eupdates,
        );
        if u_item.is_loaded() {
            svc.notify_item_unloaded(u_data, item_uid, u_item)
        }
        svc.notify_item_removed(u_data, item_uid, u_item);
        // Clear the running effects container only after services step, as they might depend on
        // having pre-removal effect list
        SolCore::util_internal_remove_stopped_reffs(u_data, item_uid, reuse_eupdates);
    }
}
