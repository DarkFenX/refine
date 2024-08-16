use crate::{
    ad,
    sol::{
        item::{SolItem, SolItemState},
        svc::SolSvcs,
        SolView,
    },
    util::StSet,
};

use super::{get_effect_powered_charge_id, is_effect_projectable, resolve_effect_status, resolve_online_effect_status};

impl SolSvcs {
    pub(in crate::sol) fn process_effects(&mut self, sol_view: &SolView, item: &SolItem, item_state: SolItemState) {
        self.process_effects_internal(sol_view, item, item_state, true);
    }
    pub(in crate::sol::svc::svce_interface) fn process_effects_internal(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        mut item_state: SolItemState,
        // Defines if container item state should be considered for charge state in special cases
        spec_charge_cont_state: bool,
    ) {
        if !item.is_loaded() {
            return;
        }
        let mut item_for_projs = item;
        // Special handling for effects of (auto)charges loaded into items which want to run charge
        // effects
        if let Some(cont_item_id) = item.get_cont_item_id() {
            for cont_effect_id in self.running_effects.iter_running(&cont_item_id) {
                let cont_effect = match sol_view.src.get_a_effect(cont_effect_id) {
                    Some(cont_effect) => cont_effect,
                    None => continue,
                };
                let charge_info = match cont_effect.charge {
                    Some(charge_info) => charge_info,
                    None => continue,
                };
                // Not interested in container item effects which don't want to run charge
                // effects
                if !charge_info.run_effects {
                    continue;
                }
                let cont_item = sol_view.items.get_item(&cont_item_id).unwrap();
                let charge_id = match charge_info.location {
                    ad::AEffectChargeLocation::Loaded => match cont_item.get_charge_id() {
                        Some(charge_id) => charge_id,
                        None => continue,
                    },
                    ad::AEffectChargeLocation::Attr(_) => match cont_item.get_autocharges() {
                        Some(autocharges) => match autocharges.get(cont_effect_id) {
                            Some(charge_id) => *charge_id,
                            None => continue,
                        },
                        None => continue,
                    },
                };
                // If we're processing effects for a charge which has an effect on container
                // which runs its effects, change context a bit
                if charge_id == item.get_id() {
                    if spec_charge_cont_state {
                        item_state = cont_item.get_state();
                    }
                    item_for_projs = cont_item;
                }
            }
        }
        let mut to_start = Vec::new();
        let mut to_stop = Vec::new();
        let mut to_process_other = StSet::new();
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
                if let Some(powered_charge_id) = get_effect_powered_charge_id(item, effect) {
                    to_process_other.insert(powered_charge_id);
                }
            } else if !running && should_run {
                to_start.push(effect.clone());
                if let Some(powered_charge_id) = get_effect_powered_charge_id(item, effect) {
                    to_process_other.insert(powered_charge_id);
                }
            };
        }
        if !to_stop.is_empty() {
            if let Some(proj_items) = item_for_projs.iter_projectee_items() {
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
        if !to_start.is_empty() {
            self.notify_effects_started(sol_view, item, &to_start);
            if let Some(projs) = item_for_projs.iter_projs() {
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
        for other_item_id in to_process_other.into_iter() {
            let other_item = sol_view.items.get_item(&other_item_id).unwrap();
            self.process_effects_internal(sol_view, other_item, other_item.get_state(), true);
        }
    }
}
