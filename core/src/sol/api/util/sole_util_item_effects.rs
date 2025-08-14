use super::is_a_effect_projectable;
use crate::{
    sol::SolarSystem,
    svc::Svc,
    ud::{UData, UEffectUpdates, UItem, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_process_effect_updates(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        eupdates: &UEffectUpdates,
    ) {
        let u_item = u_data.items.get(item_key);
        process_effect_updates(u_data, svc, item_key, u_item, eupdates);
    }
    pub(in crate::sol::api::util) fn util_internal_process_effect_updates(
        u_data: &UData,
        svc: &mut Svc,
        item_key: UItemKey,
        u_item: &UItem,
        eupdates: &UEffectUpdates,
    ) {
        process_effect_updates(u_data, svc, item_key, u_item, eupdates);
    }
}

fn process_effect_updates(
    u_data: &UData,
    svc: &mut Svc,
    item_key: UItemKey,
    u_item: &UItem,
    eupdates: &UEffectUpdates,
) {
    if !eupdates.to_start.is_empty() {
        svc.notify_effects_started(u_data, item_key, u_item, &eupdates.to_start);
        if let Some(projs) = u_item.iter_projs() {
            for (projectee_key, proj_data) in projs {
                let projectee_item = u_data.items.get(projectee_key);
                for r_effect in eupdates.to_start.iter() {
                    if is_a_effect_projectable(u_item, r_effect) {
                        svc.notify_effect_projected(
                            u_data,
                            item_key,
                            u_item,
                            r_effect,
                            projectee_key,
                            projectee_item,
                            proj_data,
                        );
                    }
                }
            }
        }
    }
    if !eupdates.to_stop.is_empty() {
        if let Some(projectee_keys) = u_item.iter_projectees() {
            for projectee_key in projectee_keys {
                let projectee_item = u_data.items.get(projectee_key);
                for r_effect in eupdates.to_stop.iter() {
                    if is_a_effect_projectable(u_item, r_effect) {
                        svc.notify_effect_unprojected(
                            u_data,
                            item_key,
                            u_item,
                            r_effect,
                            projectee_key,
                            projectee_item,
                        );
                    }
                }
            }
        }
        svc.notify_effects_stopped(u_data, item_key, u_item, &eupdates.to_stop);
    }
}
