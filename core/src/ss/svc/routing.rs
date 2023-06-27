use crate::{
    ad,
    consts::State,
    defs::{ReeId, ReeInt},
    ss::{item::SsItem, svc::SsSvcs, SsView},
};

impl SsSvcs {
    // Higher level methods
    pub(in crate::ss) fn add_item(&mut self, ss_view: &SsView, item: &SsItem) {
        let item_state = item.get_state();
        let is_a_item_loaded = item.is_loaded();
        self.notify_item_added(ss_view, item);
        if is_a_item_loaded {
            self.notify_item_loaded(ss_view, item)
        }
        let states = State::iter().filter(|v| **v <= item_state).map(|v| *v).collect();
        self.activate_item_states(ss_view, item, states);
    }
    pub(in crate::ss) fn remove_item(&mut self, ss_view: &SsView, item: &SsItem) {
        let states = State::iter().filter(|v| **v <= item.get_state()).map(|v| *v).collect();
        self.deactivate_item_states(ss_view, item, states);
        if item.is_loaded() {
            self.notify_item_unloaded(ss_view, item)
        }
        self.notify_item_removed(ss_view, item);
    }
    pub(in crate::ss) fn activate_item_states(&mut self, ss_view: &SsView, item: &SsItem, states: Vec<State>) {
        for state in states.iter() {
            self.notify_state_activated(ss_view, item, state);
        }
        if item.is_loaded() {
            for state in states.iter() {
                self.notify_state_activated_loaded(ss_view, item, state);
            }
            let item_effect_datas = item.get_effect_datas().unwrap();
            let mut starting_effects = Vec::with_capacity(item_effect_datas.len());
            for eff_id in item_effect_datas.keys() {
                match ss_view.src.get_a_effect(eff_id) {
                    Some(e) if states.contains(&e.state) => starting_effects.push(e.clone()),
                    _ => (),
                }
            }
            if !starting_effects.is_empty() {
                self.notify_effects_started(ss_view, item, &starting_effects);
            }
        };
    }
    pub(in crate::ss) fn deactivate_item_states(&mut self, ss_view: &SsView, item: &SsItem, states: Vec<State>) {
        if item.is_loaded() {
            let item_effect_datas = item.get_effect_datas().unwrap();
            let mut stopping_effects = Vec::with_capacity(item_effect_datas.len());
            for eff_id in item_effect_datas.keys() {
                match ss_view.src.get_a_effect(eff_id) {
                    Some(e) if states.contains(&e.state) => stopping_effects.push(e.clone()),
                    _ => (),
                }
            }
            if !stopping_effects.is_empty() {
                self.notify_effects_stopped(ss_view, item, &stopping_effects);
            }
            for state in states.iter() {
                self.notify_state_deactivated_loaded(ss_view, item, state);
            }
        };
        for state in states.iter() {
            self.notify_state_deactivated(ss_view, item, state);
        }
    }
    // Lower level methods
    pub(in crate::ss) fn notify_item_added(&mut self, ss_view: &SsView, item: &SsItem) {}
    pub(in crate::ss) fn notify_item_removed(&mut self, ss_view: &SsView, item: &SsItem) {}
    pub(in crate::ss) fn notify_state_activated(&mut self, ss_view: &SsView, item: &SsItem, state: &State) {}
    pub(in crate::ss) fn notify_state_deactivated(&mut self, ss_view: &SsView, item: &SsItem, state: &State) {}
    pub(in crate::ss) fn notify_item_loaded(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_item_loaded(item);
    }
    pub(in crate::ss) fn notify_item_unloaded(&mut self, ss_view: &SsView, item: &SsItem) {
        self.calc_item_unloaded(item);
    }
    pub(in crate::ss) fn notify_state_activated_loaded(&mut self, ss_view: &SsView, item: &SsItem, state: &State) {}
    pub(in crate::ss) fn notify_state_deactivated_loaded(&mut self, ss_view: &SsView, item: &SsItem, state: &State) {}
    pub(in crate::ss) fn notify_effects_started(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        self.running_effects.extend(item.get_id(), effects.iter().map(|v| v.id));
        self.calc_effects_started(ss_view, item, effects);
    }
    pub(in crate::ss) fn notify_effects_stopped(
        &mut self,
        ss_view: &SsView,
        item: &SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        self.calc_effects_stopped(ss_view, item, effects);
        self.running_effects.drain(&item.get_id(), effects.iter().map(|v| v.id));
    }
    pub(in crate::ss) fn notify_attr_val_changed(&mut self, ss_view: &SsView, item_id: &ReeId, attr_id: &ReeInt) {
        self.calc_attr_value_changed(ss_view, item_id, attr_id);
    }
}
