use crate::{
    sol::CoreInner,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItemId},
};

impl CoreInner {
    pub(in crate::api) fn util_add_stance(
        u_data: &mut UData,
        svc: &mut Svc,
        stance_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(stance_uid);
        u_item.update_reffs(reuse_eupdates, &u_data.r_data);
        CoreInner::util_add_item(u_data, svc, stance_uid, reuse_eupdates);
    }
    pub(in crate::api) fn util_remove_stance(
        u_data: &mut UData,
        svc: &mut Svc,
        stance_uid: UItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let u_item = u_data.items.get_mut(stance_uid);
        u_item.stop_all_reffs(reuse_eupdates, &u_data.r_data);
        CoreInner::util_remove_item(u_data, svc, stance_uid, reuse_eupdates);
    }
}
