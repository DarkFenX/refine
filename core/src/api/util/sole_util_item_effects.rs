use super::is_effect_projectable;
use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn util_process_effect_updates(
        u_data: &UData,
        svc: &mut Svc,
        item_uid: UItemId,
        eupdates: &UEffectUpdates,
    ) {
        let u_item = u_data.items.get(item_uid);
        process_effect_updates(u_data, svc, item_uid, u_item, eupdates);
    }
    pub(in crate::api::util) fn util_internal_process_effect_updates(
        u_data: &UData,
        svc: &mut Svc,
        item_uid: UItemId,
        u_item: &UItem,
        eupdates: &UEffectUpdates,
    ) {
        process_effect_updates(u_data, svc, item_uid, u_item, eupdates);
    }
}

fn process_effect_updates(u_data: &UData, svc: &mut Svc, item_uid: UItemId, u_item: &UItem, eupdates: &UEffectUpdates) {
    if !eupdates.to_start.is_empty() {
        svc.notify_effects_started(u_data, item_uid, u_item, &eupdates.to_start);
        if let Some(projs) = u_item.iter_projs() {
            for (projectee_uid, proj_data) in projs {
                let projectee_item = u_data.items.get(projectee_uid);
                for r_effect in eupdates.to_start.iter() {
                    if is_effect_projectable(u_item, r_effect) {
                        svc.notify_effect_projected(
                            u_data,
                            item_uid,
                            u_item,
                            r_effect,
                            projectee_uid,
                            projectee_item,
                            proj_data,
                        );
                    }
                }
            }
        }
    }
    if !eupdates.to_stop.is_empty() {
        if let Some(projectee_uids) = u_item.iter_projectees() {
            for projectee_uid in projectee_uids {
                let projectee_item = u_data.items.get(projectee_uid);
                for r_effect in eupdates.to_stop.iter() {
                    if is_effect_projectable(u_item, r_effect) {
                        svc.notify_effect_unprojected(
                            u_data,
                            item_uid,
                            u_item,
                            r_effect,
                            projectee_uid,
                            projectee_item,
                        );
                    }
                }
            }
        }
        svc.notify_effects_stopped(u_data, item_uid, u_item, &eupdates.to_stop);
    }
}
