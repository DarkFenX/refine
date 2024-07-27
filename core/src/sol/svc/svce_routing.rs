use itertools::Itertools;

use crate::{
    ad,
    defs::{AttrVal, EAttrId, SolFitId, SolItemId},
    ec,
    sol::{
        fleet::SolFleet,
        item::{SolItem, SolItemState},
        svc::SolSvcs,
        SolView,
    },
};

use super::misc::{resolve_effect_status, resolve_online_effect_status};

impl SolSvcs {
    // Higher level methods
    pub(in crate::sol) fn add_fit(&mut self, fit_id: &SolFitId) {
        self.notify_fit_added(fit_id);
    }
    pub(in crate::sol) fn remove_fit(&mut self, fit_id: &SolFitId) {
        self.notify_fit_removed(fit_id);
    }
    pub(in crate::sol) fn add_fit_to_fleet(&mut self, sol_view: &SolView, fleet: &SolFleet, fit_id: &SolFitId) {
        self.notify_fit_added_to_fleet(sol_view, fleet, fit_id);
    }
    pub(in crate::sol) fn remove_fit_from_fleet(&mut self, sol_view: &SolView, fleet: &SolFleet, fit_id: &SolFitId) {
        self.notify_fit_removed_from_fleet(sol_view, fleet, fit_id);
    }
    pub(in crate::sol) fn add_item(&mut self, sol_view: &SolView, item: &SolItem) {
        let is_a_item_loaded = item.is_loaded();
        self.notify_item_added(sol_view, item);
        if is_a_item_loaded {
            self.notify_item_loaded(sol_view, item)
        }
        self.switch_item_state(sol_view, item, SolItemState::Ghost, item.get_state());
    }
    pub(in crate::sol) fn remove_item(&mut self, sol_view: &SolView, item: &SolItem) {
        self.switch_item_state(sol_view, item, item.get_state(), SolItemState::Ghost);
        if item.is_loaded() {
            self.notify_item_unloaded(sol_view, item)
        }
        self.notify_item_removed(sol_view, item);
    }
    pub(in crate::sol) fn load_item(&mut self, sol_view: &SolView, item: &SolItem) {
        self.notify_item_loaded(sol_view, item);
        self.switch_item_state(sol_view, item, SolItemState::Ghost, item.get_state());
    }
    pub(in crate::sol) fn unload_item(&mut self, sol_view: &SolView, item: &SolItem) {
        self.switch_item_state(sol_view, item, item.get_state(), SolItemState::Ghost);
        self.notify_item_unloaded(sol_view, item)
    }
    pub(in crate::sol) fn switch_item_state(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        old_item_state: SolItemState,
        new_item_state: SolItemState,
    ) {
        if new_item_state > old_item_state {
            for state in SolItemState::iter().filter(|v| **v > old_item_state && **v <= new_item_state) {
                self.notify_state_activated(sol_view, item, state);
                if item.is_loaded() {
                    self.notify_item_state_activated_loaded(sol_view, item, state);
                }
            }
        } else if new_item_state < old_item_state {
            for state in SolItemState::iter().filter(|v| **v > new_item_state && **v <= old_item_state) {
                if item.is_loaded() {
                    self.notify_item_state_deactivated_loaded(sol_view, item, state);
                }
                self.notify_state_deactivated(sol_view, item, state);
            }
        }
        self.process_effects(sol_view, item, new_item_state);
    }
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
                to_start.push(effect.clone())
            };
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
    }
    pub(in crate::sol) fn add_item_projection(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.notify_item_projected(sol_view, projector_item, projectee_item);
        let running_effects = self.running_effects.iter_running(&projector_item.get_id());
        if !running_effects.is_empty() {
            let effect_ids = running_effects.map(|v| *v).collect_vec();
            for effect_id in effect_ids.iter() {
                let effect = sol_view.src.get_a_effect(effect_id).unwrap();
                if is_effect_projectable(effect) {
                    self.notify_effect_projected(sol_view, projector_item, effect, projectee_item, range);
                }
            }
        }
    }
    pub(in crate::sol) fn remove_item_projection(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
    ) {
        let running_effects = self.running_effects.iter_running(&projector_item.get_id());
        if !running_effects.is_empty() {
            let effect_ids = running_effects.map(|v| *v).collect_vec();
            for effect_id in effect_ids.iter() {
                let effect = sol_view.src.get_a_effect(effect_id).unwrap();
                if is_effect_projectable(effect) {
                    self.notify_effect_unprojected(sol_view, projector_item, effect, projectee_item);
                }
            }
        } else {
            drop(running_effects);
        }
        self.notify_item_unprojected(sol_view, projector_item, projectee_item);
    }
    pub(in crate::sol) fn change_item_proj_range(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.notify_item_projected(sol_view, projector_item, projectee_item);
        let running_effects = self.running_effects.iter_running(&projector_item.get_id());
        if !running_effects.is_empty() {
            let effect_ids = running_effects.map(|v| *v).collect_vec();
            for effect_id in effect_ids.iter() {
                let effect = sol_view.src.get_a_effect(effect_id).unwrap();
                if is_effect_projectable(effect) {
                    self.notify_proj_range_changed(sol_view, projector_item, effect, projectee_item, range);
                }
            }
        }
    }
    // Lower level methods
    fn notify_fit_added(&mut self, fit_id: &SolFitId) {
        self.calc_fit_added(fit_id);
    }
    fn notify_fit_removed(&mut self, fit_id: &SolFitId) {
        self.calc_fit_removed(fit_id);
    }
    fn notify_fit_added_to_fleet(&mut self, sol_view: &SolView, fleet: &SolFleet, fit_id: &SolFitId) {
        self.calc_fit_added_to_fleet(sol_view, fleet, fit_id);
    }
    fn notify_fit_removed_from_fleet(&mut self, sol_view: &SolView, fleet: &SolFleet, fit_id: &SolFitId) {
        self.calc_fit_removed_from_fleet(sol_view, fleet, fit_id);
    }
    fn notify_item_added(&mut self, sol_view: &SolView, item: &SolItem) {
        self.calc_item_added(sol_view, item);
    }
    fn notify_item_removed(&mut self, sol_view: &SolView, item: &SolItem) {
        self.calc_item_removed(sol_view, item);
    }
    fn notify_state_activated(&mut self, sol_view: &SolView, item: &SolItem, state: &SolItemState) {}
    fn notify_state_deactivated(&mut self, sol_view: &SolView, item: &SolItem, state: &SolItemState) {}
    fn notify_item_loaded(&mut self, sol_view: &SolView, item: &SolItem) {
        self.calc_item_loaded(sol_view, item);
    }
    fn notify_item_unloaded(&mut self, sol_view: &SolView, item: &SolItem) {
        self.calc_item_unloaded(sol_view, item);
    }
    fn notify_item_state_activated_loaded(&mut self, sol_view: &SolView, item: &SolItem, state: &SolItemState) {}
    fn notify_item_state_deactivated_loaded(&mut self, sol_view: &SolView, item: &SolItem, state: &SolItemState) {}
    fn notify_effects_started(&mut self, sol_view: &SolView, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        self.running_effects
            .effects_started(item.get_id(), effects.iter().map(|v| v.id));
        self.calc_effects_started(sol_view, item, effects);
    }
    fn notify_effects_stopped(&mut self, sol_view: &SolView, item: &SolItem, effects: &Vec<ad::ArcEffect>) {
        self.calc_effects_stopped(sol_view, item, effects);
        self.running_effects
            .effects_stopped(&item.get_id(), effects.iter().map(|v| &v.id));
    }
    pub(in crate::sol) fn notify_item_projected(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
    ) {
    }
    pub(in crate::sol) fn notify_item_unprojected(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
    ) {
    }
    pub(in crate::sol) fn notify_effect_projected(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        effect: &ad::ArcEffect,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.calc_effect_projected(sol_view, projector_item, effect, projectee_item, range);
    }
    pub(in crate::sol) fn notify_effect_unprojected(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        effect: &ad::ArcEffect,
        projectee_item: &SolItem,
    ) {
        self.calc_effect_unprojected(sol_view, projector_item, effect, projectee_item);
    }
    pub(in crate::sol) fn notify_proj_range_changed(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        effect: &ad::ArcEffect,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.calc_effect_proj_range_changed(sol_view, projector_item, effect, projectee_item, range);
    }
    pub(super) fn notify_attr_val_changed(&mut self, sol_view: &SolView, item_id: &SolItemId, attr_id: &EAttrId) {
        self.calc_attr_value_changed(sol_view, item_id, attr_id);
    }
}

fn is_effect_projectable(effect: &ad::AEffect) -> bool {
    effect.category == ec::effcats::TARGET || effect.category == ec::effcats::SYSTEM || effect.buff.is_some()
}
