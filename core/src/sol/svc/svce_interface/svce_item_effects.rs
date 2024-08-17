use crate::sol::{
    item::{SolItem, SolItemState},
    svc::SolSvcs,
    SolView,
};

use super::{is_effect_projectable, resolve_effect_status, resolve_online_effect_status};

impl SolSvcs {
    pub(in crate::sol) fn process_effects(&mut self, sol_view: &SolView, item: &SolItem, item_state: SolItemState) {
        if !item.is_loaded() {
            return;
        }
        let mut to_start = Vec::new();
        let mut to_stop = Vec::new();
        let online_should_run = resolve_online_effect_status(sol_view, item, item_state);
        for effect_id in item.get_effect_datas().unwrap().keys() {
            let effect = match sol_view.src.get_a_effect(effect_id) {
                Some(e) => e,
                None => continue,
            };
            let should_run = resolve_effect_status(item, item_state, effect, online_should_run);
            let running = self.running_effects.is_running(&item.get_id(), effect_id);
            if running && !should_run {
                to_stop.push(effect.clone());
            } else if !running && should_run {
                to_start.push(effect.clone());
            };
        }
        if !to_start.is_empty() {
            self.notify_effects_started(sol_view, item, &to_start);
            if let Some(projs) = item.iter_projs() {
                for (proj_item_id, range) in projs {
                    let proj_item = sol_view.items.get_item(proj_item_id).unwrap();
                    for effect in to_start.iter() {
                        if is_effect_projectable(effect) {
                            self.notify_effect_projected(sol_view, item, effect, proj_item, *range);
                        }
                    }
                }
            }
        }
        if !to_stop.is_empty() {
            if let Some(proj_items) = item.iter_projectee_items() {
                for proj_item_id in proj_items {
                    let proj_item = sol_view.items.get_item(proj_item_id).unwrap();
                    for effect in to_stop.iter() {
                        if is_effect_projectable(effect) {
                            self.notify_effect_unprojected(sol_view, item, effect, proj_item);
                        }
                    }
                }
            }
            self.notify_effects_stopped(sol_view, item, &to_stop);
        }
    }
}
