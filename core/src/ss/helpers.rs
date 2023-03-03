use crate::{consts::State, Src};

use super::{calc::CalcSvc, item::Item, notify};

pub(in crate::ss) fn add_item(item: &Item, src: &Src, calc: &mut CalcSvc) {
    let item_state = item.get_state();
    let is_citem_loaded = item.is_loaded();
    notify::item_added(item);
    if is_citem_loaded {
        notify::item_loaded(item, calc)
    }
    match item_state {
        State::Offline => {
            let states = vec![State::Offline];
            activate_item_states(item, states, src, calc);
        }
        State::Online => {
            let states = vec![State::Offline, State::Online];
            activate_item_states(item, states, src, calc);
        }
        State::Active => {
            let states = vec![State::Offline, State::Online, State::Active];
            activate_item_states(item, states, src, calc);
        }
        State::Overload => {
            let states = vec![State::Offline, State::Online, State::Active, State::Overload];
            activate_item_states(item, states, src, calc);
        }
        _ => (),
    }
}
pub(in crate::ss) fn remove_item(item: &Item, src: &Src, calc: &mut CalcSvc) {
    match item.get_state() {
        State::Offline => {
            let states = vec![State::Offline];
            deactivate_item_states(item, states, src, calc);
        }
        State::Online => {
            let states = vec![State::Online, State::Offline];
            deactivate_item_states(item, states, src, calc);
        }
        State::Active => {
            let states = vec![State::Active, State::Online, State::Offline];
            deactivate_item_states(item, states, src, calc);
        }
        State::Overload => {
            let states = vec![State::Overload, State::Active, State::Online, State::Offline];
            deactivate_item_states(item, states, src, calc);
        }
        _ => (),
    }
    if item.is_loaded() {
        notify::item_unloaded(item, calc)
    }
    notify::item_removed(item);
}
pub(in crate::ss) fn activate_item_states(item: &Item, states: Vec<State>, src: &Src, calc: &mut CalcSvc) {
    for state in states.iter() {
        notify::state_activated(item, state);
    }
    if item.is_loaded() {
        for state in states.iter() {
            notify::state_activated_loaded(item, state);
        }
        let item_effect_datas = item.get_effect_datas().unwrap();
        for eff_id in item_effect_datas.keys() {
            let mut starting_effects = Vec::with_capacity(item_effect_datas.len());
            match src.cache_handler.get_effect(eff_id) {
                Some(e) if states.contains(&e.state) => starting_effects.push(e.clone()),
                _ => (),
            }
            if !starting_effects.is_empty() {
                notify::effects_started(item, &starting_effects, calc);
            }
        }
    };
}
pub(in crate::ss) fn deactivate_item_states(item: &Item, states: Vec<State>, src: &Src, calc: &mut CalcSvc) {
    if item.is_loaded() {
        let item_effect_datas = item.get_effect_datas().unwrap();
        for eff_id in item_effect_datas.keys() {
            let mut stopping_effects = Vec::with_capacity(item_effect_datas.len());
            match src.cache_handler.get_effect(eff_id) {
                Some(e) if states.contains(&e.state) => stopping_effects.push(e.clone()),
                _ => (),
            }
            if !stopping_effects.is_empty() {
                notify::effects_stopped(item, &stopping_effects, calc);
            }
        }
        for state in states.iter() {
            notify::state_deactivated_loaded(item, state);
        }
    };
    for state in states.iter() {
        notify::state_deactivated(item, state);
    }
}
