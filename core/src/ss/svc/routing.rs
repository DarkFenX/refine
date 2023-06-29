use crate::{
    ad,
    consts::{effects, EffectMode, State},
    defs::{EAttrId, SsItemId},
    ss::{item::SsItem, svc::SsSvcs, SsView},
};

impl SsSvcs {
    // Higher level methods
    pub(in crate::ss) fn add_item(&mut self, ss_view: &SsView, item: &SsItem) {
        let is_a_item_loaded = item.is_loaded();
        self.notify_item_added(ss_view, item);
        if is_a_item_loaded {
            self.notify_item_loaded(ss_view, item)
        }
        self.switch_state(ss_view, item, State::Ghost, item.get_state());
    }
    pub(in crate::ss) fn remove_item(&mut self, ss_view: &SsView, item: &SsItem) {
        self.switch_state(ss_view, item, item.get_state(), State::Ghost);
        if item.is_loaded() {
            self.notify_item_unloaded(ss_view, item)
        }
        self.notify_item_removed(ss_view, item);
    }
    pub(in crate::ss) fn switch_state(&mut self, ss_view: &SsView, item: &SsItem, old_state: State, new_state: State) {
        if new_state > old_state {
            for state in State::iter().filter(|v| **v > old_state && **v <= new_state) {
                self.notify_state_activated(ss_view, item, state);
                if item.is_loaded() {
                    self.notify_state_activated_loaded(ss_view, item, state);
                }
            }
        } else if new_state < old_state {
            for state in State::iter().filter(|v| **v > new_state && **v <= old_state) {
                if item.is_loaded() {
                    self.notify_state_deactivated_loaded(ss_view, item, state);
                }
                self.notify_state_deactivated(ss_view, item, state);
            }
        }
        self.process_effects(ss_view, item, new_state);
    }
    pub(in crate::ss) fn process_effects(&mut self, ss_view: &SsView, item: &SsItem, state: State) {
        if !item.is_loaded() {
            return;
        }
        let mut to_start = Vec::new();
        let mut to_stop = Vec::new();
        let effect_modes = item.get_effect_modes();
        let online_should_run = resolve_online_run_status(ss_view, item, state);
        for effect_id in item.get_effect_datas().unwrap().keys() {
            let effect = match ss_view.src.get_a_effect(effect_id) {
                Some(e) => e,
                None => continue,
            };
            let should_run = match effect_modes.get(effect_id) {
                EffectMode::FullCompliance => resolve_full_compliance(item, state, &effect, online_should_run),
                EffectMode::StateCompliance => state >= effect.state,
                EffectMode::ForceRun => true,
                EffectMode::ForceStop => false,
            };
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
    fn notify_effects_started(&mut self, ss_view: &SsView, item: &SsItem, effects: &Vec<ad::ArcEffect>) {
        self.running_effects
            .effects_started(item.get_id(), effects.iter().map(|v| v.id));
        self.calc_effects_started(ss_view, item, effects);
    }
    fn notify_effects_stopped(&mut self, ss_view: &SsView, item: &SsItem, effects: &Vec<ad::ArcEffect>) {
        self.calc_effects_stopped(ss_view, item, effects);
        self.running_effects
            .effects_stopped(&item.get_id(), effects.iter().map(|v| v.id));
    }
    pub(in crate::ss) fn notify_attr_val_changed(&mut self, ss_view: &SsView, item_id: &SsItemId, attr_id: &EAttrId) {
        self.calc_attr_value_changed(ss_view, item_id, attr_id);
    }
}

fn resolve_online_run_status(ss_view: &SsView, item: &SsItem, state: State) -> bool {
    if !item.get_effect_datas().unwrap().contains_key(&effects::ONLINE) {
        return false;
    }
    let effect = match ss_view.src.get_a_effect(&effects::ONLINE) {
        Some(effect) => effect,
        None => return false,
    };
    resolve_full_compliance(item, state, &effect, false)
}

fn resolve_full_compliance(item: &SsItem, state: State, effect: &ad::ArcEffect, online_running: bool) -> bool {
    // State compliance is enforced for all states
    if effect.state > state {
        return false;
    };
    match effect.state {
        // Offline effects must not specify fitting usage chance
        // (not to run booster side effects by default)
        State::Offline => effect.chance_attr_id.is_none(),
        // Online effects depend on 'online' effect
        State::Online => {
            // Online effect itself runs unconditionally
            if effect.id == effects::ONLINE {
                true
            // Other online effects run only if online effect is running
            } else {
                online_running
            }
        }
        // Only default active effect is run in full compliance
        State::Active => match item.get_defeff_id() {
            Ok(defeff_id_opt) => match defeff_id_opt {
                Some(defeff_id) => *defeff_id == effect.id,
                _ => false,
            },
            _ => false,
        },
        // No additional restrictions for overload effects
        State::Overload => true,
        // None of effects should have ghost state, this is custom state for items
        State::Ghost => false,
    }
}
