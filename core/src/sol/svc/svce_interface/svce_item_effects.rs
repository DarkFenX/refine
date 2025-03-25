use crate::{
    ad,
    sol::{
        svc::Svc,
        uad::{Uad, item::Item},
    },
};

use super::{is_a_effect_projectable, resolve_effect_status, resolve_online_effect_status};

impl Svc {
    pub(in crate::sol) fn process_effects(&mut self, uad: &Uad, item: &Item, item_a_state: ad::AState) {
        if !item.is_loaded() {
            return;
        }
        let mut to_start = Vec::new();
        let mut to_stop = Vec::new();
        let online_should_run = resolve_online_effect_status(uad, item, item_a_state);
        for a_effect_id in item.get_a_effect_datas().unwrap().keys() {
            let a_effect = match uad.src.get_a_effect(a_effect_id) {
                Some(a_effect) => a_effect,
                None => continue,
            };
            let should_run = resolve_effect_status(item, item_a_state, a_effect, online_should_run);
            let running = self.running_effects.is_running(&item.get_item_id(), a_effect_id);
            if running && !should_run {
                to_stop.push(a_effect.clone());
            } else if !running && should_run {
                to_start.push(a_effect.clone());
            };
        }
        if !to_start.is_empty() {
            self.notify_effects_started(uad, item, &to_start);
            if let Some(projs) = item.iter_projs() {
                for (proj_item_id, range) in projs {
                    let proj_item = uad.items.get_item(proj_item_id).unwrap();
                    for a_effect in to_start.iter() {
                        if is_a_effect_projectable(a_effect) {
                            self.notify_effect_projected(uad, item, a_effect, proj_item, *range);
                        }
                    }
                }
            }
        }
        if !to_stop.is_empty() {
            if let Some(proj_items) = item.iter_projectee_items() {
                for proj_item_id in proj_items {
                    let proj_item = uad.items.get_item(proj_item_id).unwrap();
                    for a_effect in to_stop.iter() {
                        if is_a_effect_projectable(a_effect) {
                            self.notify_effect_unprojected(uad, item, a_effect, proj_item);
                        }
                    }
                }
            }
            self.notify_effects_stopped(uad, item, &to_stop);
        }
    }
}
