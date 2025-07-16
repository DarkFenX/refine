use super::is_a_effect_projectable;
use crate::{
    def::ItemKey,
    sol::SolarSystem,
    svc::Svc,
    uad::{Uad, UadEffectUpdates, UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_process_effect_updates(
        uad: &Uad,
        svc: &mut Svc,
        item_key: ItemKey,
        uad_item: &UadItem,
        eupdates: &UadEffectUpdates,
    ) {
        process_effect_updates(uad, svc, item_key, uad_item, eupdates, true);
    }
    pub(in crate::sol::api::util) fn util_internal_process_effect_updates_without_projs(
        uad: &Uad,
        svc: &mut Svc,
        item_key: ItemKey,
        uad_item: &UadItem,
        eupdates: &UadEffectUpdates,
    ) {
        process_effect_updates(uad, svc, item_key, uad_item, eupdates, false);
    }
}

fn process_effect_updates(
    uad: &Uad,
    svc: &mut Svc,
    item_key: ItemKey,
    uad_item: &UadItem,
    eupdates: &UadEffectUpdates,
    handle_projs: bool,
) {
    if !eupdates.to_start.is_empty() {
        svc.notify_effects_started(uad, item_key, uad_item, &eupdates.to_start);
        if handle_projs && let Some(projs) = uad_item.iter_projs() {
            for (projectee_key, range) in projs {
                let projectee_item = uad.items.get(projectee_key);
                for a_effect in eupdates.to_start.iter() {
                    if is_a_effect_projectable(uad_item, a_effect) {
                        svc.notify_effect_projected(
                            uad,
                            item_key,
                            uad_item,
                            a_effect,
                            projectee_key,
                            projectee_item,
                            range,
                        );
                    }
                }
            }
        }
    }
    if !eupdates.to_stop.is_empty() {
        if handle_projs && let Some(projectee_keys) = uad_item.iter_projectees() {
            for projectee_key in projectee_keys {
                let projectee_item = uad.items.get(projectee_key);
                for a_effect in eupdates.to_stop.iter() {
                    if is_a_effect_projectable(uad_item, a_effect) {
                        svc.notify_effect_unprojected(uad, item_key, uad_item, a_effect, projectee_key, projectee_item);
                    }
                }
            }
        }
        svc.notify_effects_stopped(uad, item_key, uad_item, &eupdates.to_stop);
    }
}
