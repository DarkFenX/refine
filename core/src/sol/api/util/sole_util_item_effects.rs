use itertools::Itertools;

use super::{is_a_effect_projectable, resolve_effect_status, resolve_online_effect_status};
use crate::{
    ad,
    def::ItemKey,
    sol::{SolarSystem, reffs::REffs},
    svc::Svc,
    uad::{Uad, UadItem},
};

impl SolarSystem {
    pub(in crate::sol::api) fn util_process_effects(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        item_key: ItemKey,
        uad_item: &UadItem,
        item_a_state: ad::AState,
    ) {
        process_effects(uad, svc, reffs, item_key, uad_item, item_a_state, true);
    }
    pub(in crate::sol::api::util) fn util_internal_process_effects_without_projs(
        uad: &Uad,
        svc: &mut Svc,
        reffs: &mut REffs,
        item_key: ItemKey,
        uad_item: &UadItem,
        item_a_state: ad::AState,
    ) {
        process_effects(uad, svc, reffs, item_key, uad_item, item_a_state, false);
    }
}

fn process_effects(
    uad: &Uad,
    svc: &mut Svc,
    reffs: &mut REffs,
    item_key: ItemKey,
    uad_item: &UadItem,
    item_a_state: ad::AState,
    handle_projs: bool,
) {
    if !uad_item.is_loaded() {
        return;
    }
    match item_a_state {
        ad::AState::Ghost => stop_all_effects(uad, svc, reffs, item_key, uad_item, handle_projs),
        _ => update_running_effects(uad, svc, reffs, item_key, uad_item, item_a_state, handle_projs),
    }
}

fn stop_all_effects(
    uad: &Uad,
    svc: &mut Svc,
    reffs: &mut REffs,
    item_key: ItemKey,
    uad_item: &UadItem,
    handle_projs: bool,
) {
    // We don't want waste time resolving effects when we want them to just stop (which happens
    // before e.g. item removal)
    let to_stop = match reffs.extract_running(&item_key) {
        Some(running_a_effect_ids) => running_a_effect_ids
            .map(|a_effect_id| uad.src.get_a_effect(&a_effect_id).unwrap().clone())
            .collect_vec(),
        None => return,
    };
    process_stop_start(uad, svc, item_key, uad_item, Vec::new(), to_stop, handle_projs)
}

fn update_running_effects(
    uad: &Uad,
    svc: &mut Svc,
    reffs: &mut REffs,
    item_key: ItemKey,
    uad_item: &UadItem,
    item_a_state: ad::AState,
    handle_projs: bool,
) {
    let mut to_start = Vec::new();
    let mut to_stop = Vec::new();
    let online_should_run = resolve_online_effect_status(uad, uad_item, item_a_state);
    for a_effect_id in uad_item.get_a_effect_datas().unwrap().keys() {
        let a_effect = match uad.src.get_a_effect(a_effect_id) {
            Some(a_effect) => a_effect,
            None => continue,
        };
        let should_run = resolve_effect_status(uad_item, item_a_state, a_effect, online_should_run);
        let running = reffs.is_running(&item_key, a_effect_id);
        if running && !should_run {
            to_stop.push(a_effect.clone());
        } else if !running && should_run {
            to_start.push(a_effect.clone());
        };
    }
    reffs.effects_started(item_key, to_start.iter().map(|a_effect| a_effect.ae.id));
    reffs.effects_stopped(&item_key, to_stop.iter().map(|a_effect| &a_effect.ae.id));
    process_stop_start(uad, svc, item_key, uad_item, to_start, to_stop, handle_projs)
}

fn process_stop_start(
    uad: &Uad,
    svc: &mut Svc,
    item_key: ItemKey,
    uad_item: &UadItem,
    to_start: Vec<ad::ArcEffectRt>,
    to_stop: Vec<ad::ArcEffectRt>,
    handle_projs: bool,
) {
    if !to_start.is_empty() {
        svc.notify_effects_started(uad, item_key, uad_item, &to_start);
        if handle_projs && let Some(projs) = uad_item.iter_projs() {
            for (&projectee_item_key, range) in projs {
                let projectee_item = uad.items.get(projectee_item_key);
                for a_effect in to_start.iter() {
                    if is_a_effect_projectable(uad_item, a_effect) {
                        svc.notify_effect_projected(
                            uad,
                            item_key,
                            uad_item,
                            a_effect,
                            projectee_item_key,
                            projectee_item,
                            *range,
                        );
                    }
                }
            }
        }
    }
    if !to_stop.is_empty() {
        if handle_projs && let Some(projectee_item_keys) = uad_item.iter_projectee_item_keys() {
            for &projectee_item_key in projectee_item_keys {
                let projectee_item = uad.items.get(projectee_item_key);
                for a_effect in to_stop.iter() {
                    if is_a_effect_projectable(uad_item, a_effect) {
                        svc.notify_effect_unprojected(
                            uad,
                            item_key,
                            uad_item,
                            a_effect,
                            projectee_item_key,
                            projectee_item,
                        );
                    }
                }
            }
        }
        svc.notify_effects_stopped(uad, item_key, uad_item, &to_stop);
    }
}
