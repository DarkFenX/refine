use crate::{
    ad,
    defs::{EAttrId, SsFitId, SsItemId},
    shr::State,
    ss::{fleet::SsFleet, item::SsItem, svc::SsSvcs, SsView},
};

use super::misc::{resolve_effect_status, resolve_online_effect_status};

impl SsSvcs {
    // Higher level methods
    pub(in crate::ss) fn add_fit(&mut self, fit_id: &SsFitId) {
        self.notify_fit_added(fit_id);
    }
    pub(in crate::ss) fn remove_fit(&mut self, fit_id: &SsFitId) {
        self.notify_fit_removed(fit_id);
    }
    pub(in crate::ss) fn add_fit_to_fleet(&mut self, ss_view: &SsView, fleet: &SsFleet, fit_id: &SsFitId) {
        self.notify_fit_added_to_fleet(ss_view, fleet, fit_id);
    }
    pub(in crate::ss) fn remove_fit_from_fleet(&mut self, ss_view: &SsView, fleet: &SsFleet, fit_id: &SsFitId) {
        self.notify_fit_removed_from_fleet(ss_view, fleet, fit_id);
    }
    pub(in crate::ss) fn add_item(&mut self, ss_view: &SsView, item: &SsItem) {
        let is_a_item_loaded = item.is_loaded();
        self.notify_item_added(ss_view, item);
        if is_a_item_loaded {
            self.notify_item_loaded(ss_view, item)
        }
        self.switch_item_state(ss_view, item, State::Ghost, item.get_state());
    }
    pub(in crate::ss) fn remove_item(&mut self, ss_view: &SsView, item: &SsItem) {
        self.switch_item_state(ss_view, item, item.get_state(), State::Ghost);
        if item.is_loaded() {
            self.notify_item_unloaded(ss_view, item)
        }
        self.notify_item_removed(ss_view, item);
    }
    pub(in crate::ss) fn switch_item_state(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        old_item_state: State,
        new_item_state: State,
    ) {
        if new_item_state > old_item_state {
            for state in State::iter().filter(|v| **v > old_item_state && **v <= new_item_state) {
                self.notify_state_activated(ss_view, item, state);
                if item.is_loaded() {
                    self.notify_item_state_activated_loaded(ss_view, item, state);
                }
            }
        } else if new_item_state < old_item_state {
            for state in State::iter().filter(|v| **v > new_item_state && **v <= old_item_state) {
                if item.is_loaded() {
                    self.notify_item_state_deactivated_loaded(ss_view, item, state);
                }
                self.notify_state_deactivated(ss_view, item, state);
            }
        }
        self.process_effects(ss_view, item, new_item_state);
    }
    pub(in crate::ss) fn process_effects(&mut self, ss_view: &SsView, item: &SsItem, item_state: State) {
        if !item.is_loaded() {
            return;
        }
        let mut to_start = Vec::new();
        let mut to_stop = Vec::new();
        let online_should_run = resolve_online_effect_status(ss_view, item, item_state);
        for effect_id in item.get_effect_datas().unwrap().keys() {
            let effect = match ss_view.src.get_a_effect(effect_id) {
                Some(e) => e,
                None => continue,
            };
            let should_run = resolve_effect_status(item, item_state, &effect, online_should_run);
            let running = self.running_effects.is_running(&item.get_id(), effect_id);
            if running && !should_run {
                to_stop.push(effect);
            } else if !running && should_run {
                to_start.push(effect)
            };
        }
        if !to_stop.is_empty() {
            self.notify_effects_stopped(ss_view, item, &to_stop)
        }
        if !to_start.is_empty() {
            self.notify_effects_started(ss_view, item, &to_start)
        }
    }
    pub(in crate::ss) fn add_item_tgt(&mut self, ss_view: &SsView, item: &SsItem, tgt_item_id: SsItemId) {
        self.notify_item_tgt_added(ss_view, item, tgt_item_id);
    }
    pub(in crate::ss) fn remove_item_tgt(&mut self, ss_view: &SsView, item: &SsItem, tgt_item_id: &SsItemId) {
        self.notify_item_tgt_removed(ss_view, item, tgt_item_id);
    }
    // Lower level methods
    fn notify_fit_added(&mut self, fit_id: &SsFitId) {
        self.calc_fit_added(fit_id);
    }
    fn notify_fit_removed(&mut self, fit_id: &SsFitId) {
        self.calc_fit_removed(fit_id);
    }
    fn notify_fit_added_to_fleet(&mut self, ss_view: &SsView, fleet: &SsFleet, fit_id: &SsFitId) {
        self.calc_fit_added_to_fleet(ss_view, fleet, fit_id);
    }
    fn notify_fit_removed_from_fleet(&mut self, ss_view: &SsView, fleet: &SsFleet, fit_id: &SsFitId) {
        self.calc_fit_removed_from_fleet(ss_view, fleet, fit_id);
    }
    fn notify_item_added(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_item_added(ss_view, item);
    }
    fn notify_item_removed(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_item_removed(ss_view, item);
    }
    fn notify_state_activated(&mut self, ss_view: &SsView, item: &SsItem, state: &State) {}
    fn notify_state_deactivated(&mut self, ss_view: &SsView, item: &SsItem, state: &State) {}
    fn notify_item_loaded(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_item_loaded(ss_view, item);
    }
    fn notify_item_unloaded(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_item_unloaded(ss_view, item);
    }
    fn notify_item_state_activated_loaded(&mut self, ss_view: &SsView, item: &SsItem, state: &State) {}
    fn notify_item_state_deactivated_loaded(&mut self, ss_view: &SsView, item: &SsItem, state: &State) {}
    fn notify_effects_started(&mut self, ss_view: &SsView, item: &SsItem, effects: &Vec<ad::ArcEffect>) {
        self.running_effects
            .effects_started(item.get_id(), effects.iter().map(|v| v.id));
        self.calc_effects_started(ss_view, item, effects);
    }
    fn notify_effects_stopped(&mut self, ss_view: &SsView, item: &SsItem, effects: &Vec<ad::ArcEffect>) {
        self.calc_effects_stopped(ss_view, item, effects);
        self.running_effects
            .effects_stopped(&item.get_id(), effects.iter().map(|v| &v.id));
    }
    pub(in crate::ss) fn notify_item_tgt_added(&mut self, ss_view: &SsView, item: &SsItem, tgt_item_id: SsItemId) {
        self.calc_item_tgt_added(ss_view, item, tgt_item_id);
    }
    pub(in crate::ss) fn notify_item_tgt_removed(&mut self, ss_view: &SsView, item: &SsItem, tgt_item_id: &SsItemId) {
        self.calc_item_tgt_removed(ss_view, item, tgt_item_id);
    }
    pub(super) fn notify_attr_val_changed(&mut self, ss_view: &SsView, item_id: &SsItemId, attr_id: &EAttrId) {
        self.calc_attr_value_changed(ss_view, item_id, attr_id);
    }
}
