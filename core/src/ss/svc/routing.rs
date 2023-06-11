use crate::{
    ad,
    consts::State,
    defs::ReeId,
    ss::{svc::SsSvcs, SsView},
    ssi,
};

impl SsSvcs {
    // Higher level methods
    pub(in crate::ss) fn add_item(&mut self, ss_view: &SsView, item: &ssi::SsItem) {
        let item_state = item.get_state();
        let is_a_item_loaded = item.is_loaded();
        self.notify_item_added(ss_view, item);
        if is_a_item_loaded {
            self.notify_item_loaded(ss_view, item)
        }
        match item_state {
            State::Offline => {
                let states = vec![State::Offline];
                self.activate_item_states(ss_view, item, states);
            }
            State::Online => {
                let states = vec![State::Offline, State::Online];
                self.activate_item_states(ss_view, item, states);
            }
            State::Active => {
                let states = vec![State::Offline, State::Online, State::Active];
                self.activate_item_states(ss_view, item, states);
            }
            State::Overload => {
                let states = vec![State::Offline, State::Online, State::Active, State::Overload];
                self.activate_item_states(ss_view, item, states);
            }
            _ => (),
        }
    }
    pub(in crate::ss) fn remove_item(&mut self, ss_view: &SsView, item: &ssi::SsItem) {
        match item.get_state() {
            State::Offline => {
                let states = vec![State::Offline];
                self.deactivate_item_states(ss_view, item, states);
            }
            State::Online => {
                let states = vec![State::Online, State::Offline];
                self.deactivate_item_states(ss_view, item, states);
            }
            State::Active => {
                let states = vec![State::Active, State::Online, State::Offline];
                self.deactivate_item_states(ss_view, item, states);
            }
            State::Overload => {
                let states = vec![State::Overload, State::Active, State::Online, State::Offline];
                self.deactivate_item_states(ss_view, item, states);
            }
            _ => (),
        }
        if item.is_loaded() {
            self.notify_item_unloaded(ss_view, item)
        }
        self.notify_item_removed(ss_view, item);
    }
    pub(in crate::ss) fn activate_item_states(&mut self, ss_view: &SsView, item: &ssi::SsItem, states: Vec<State>) {
        for state in states.iter() {
            self.notify_state_activated(ss_view, item, state);
        }
        if item.is_loaded() {
            for state in states.iter() {
                self.notify_state_activated_loaded(ss_view, item, state);
            }
            let item_effect_datas = item.get_effect_datas().unwrap();
            for eff_id in item_effect_datas.keys() {
                let mut starting_effects = Vec::with_capacity(item_effect_datas.len());
                match ss_view.src.get_a_effect(eff_id) {
                    Some(e) if states.contains(&e.state) => starting_effects.push(e.clone()),
                    _ => (),
                }
                if !starting_effects.is_empty() {
                    self.notify_effects_started(ss_view, item, &starting_effects);
                }
            }
        };
    }
    pub(in crate::ss) fn deactivate_item_states(&mut self, ss_view: &SsView, item: &ssi::SsItem, states: Vec<State>) {
        if item.is_loaded() {
            let item_effect_datas = item.get_effect_datas().unwrap();
            for eff_id in item_effect_datas.keys() {
                let mut stopping_effects = Vec::with_capacity(item_effect_datas.len());
                match ss_view.src.get_a_effect(eff_id) {
                    Some(e) if states.contains(&e.state) => stopping_effects.push(e.clone()),
                    _ => (),
                }
                if !stopping_effects.is_empty() {
                    self.notify_effects_stopped(ss_view, item, &stopping_effects);
                }
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
    pub(in crate::ss) fn notify_item_added(&mut self, ss_view: &SsView, item: &ssi::SsItem) {}
    pub(in crate::ss) fn notify_item_removed(&mut self, ss_view: &SsView, item: &ssi::SsItem) {}
    pub(in crate::ss) fn notify_state_activated(&mut self, ss_view: &SsView, item: &ssi::SsItem, state: &State) {}
    pub(in crate::ss) fn notify_state_deactivated(&mut self, ss_view: &SsView, item: &ssi::SsItem, state: &State) {}
    pub(in crate::ss) fn notify_item_loaded(&mut self, ss_view: &SsView, item: &ssi::SsItem) {
        self.calc.item_loaded(item);
    }
    pub(in crate::ss) fn notify_item_unloaded(&mut self, ss_view: &SsView, item: &ssi::SsItem) {
        self.calc.item_unloaded(item);
    }
    pub(in crate::ss) fn notify_state_activated_loaded(&mut self, ss_view: &SsView, item: &ssi::SsItem, state: &State) {
    }
    pub(in crate::ss) fn notify_state_deactivated_loaded(
        &mut self,
        ss_view: &SsView,
        item: &ssi::SsItem,
        state: &State,
    ) {
    }
    pub(in crate::ss) fn notify_effects_started(
        &mut self,
        ss_view: &SsView,
        item: &ssi::SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        self.calc.effects_started(item, effects, &ss_view.items);
    }
    pub(in crate::ss) fn notify_effects_stopped(
        &mut self,
        ss_view: &SsView,
        item: &ssi::SsItem,
        effects: &Vec<ad::ArcEffect>,
    ) {
        self.calc.effects_stopped(item, effects, &ss_view.items);
    }
    pub(in crate::ss) fn notify_attr_val_changed(&mut self, ss_view: &SsView, item: &ssi::SsItem, attr_id: ReeId) {}
}
