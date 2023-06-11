use crate::{consts::State, ssi};

use super::{notify, SsInnerData};

pub(in crate::ss) fn add_item(item: &ssi::SsItem, ss_data: &mut SsInnerData) {
    let item_state = item.get_state();
    let is_citem_loaded = item.is_loaded();
    notify::item_added(item);
    if is_citem_loaded {
        notify::item_loaded(item, ss_data.calc)
    }
    match item_state {
        State::Offline => {
            let states = vec![State::Offline];
            activate_item_states(item, states, ss_data);
        }
        State::Online => {
            let states = vec![State::Offline, State::Online];
            activate_item_states(item, states, ss_data);
        }
        State::Active => {
            let states = vec![State::Offline, State::Online, State::Active];
            activate_item_states(item, states, ss_data);
        }
        State::Overload => {
            let states = vec![State::Offline, State::Online, State::Active, State::Overload];
            activate_item_states(item, states, ss_data);
        }
        _ => (),
    }
}
pub(in crate::ss) fn remove_item(item: &ssi::SsItem, ss_data: &mut SsInnerData) {
    match item.get_state() {
        State::Offline => {
            let states = vec![State::Offline];
            deactivate_item_states(item, states, ss_data);
        }
        State::Online => {
            let states = vec![State::Online, State::Offline];
            deactivate_item_states(item, states, ss_data);
        }
        State::Active => {
            let states = vec![State::Active, State::Online, State::Offline];
            deactivate_item_states(item, states, ss_data);
        }
        State::Overload => {
            let states = vec![State::Overload, State::Active, State::Online, State::Offline];
            deactivate_item_states(item, states, ss_data);
        }
        _ => (),
    }
    if item.is_loaded() {
        notify::item_unloaded(item, ss_data.calc)
    }
    notify::item_removed(item);
}
pub(in crate::ss) fn activate_item_states(item: &ssi::SsItem, states: Vec<State>, ss_data: &mut SsInnerData) {
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
            match ss_data.src.get_a_effect(eff_id) {
                Some(e) if states.contains(&e.state) => starting_effects.push(e.clone()),
                _ => (),
            }
            if !starting_effects.is_empty() {
                notify::effects_started(item, &starting_effects, ss_data.items, ss_data.calc);
            }
        }
    };
}
pub(in crate::ss) fn deactivate_item_states(item: &ssi::SsItem, states: Vec<State>, ss_data: &mut SsInnerData) {
    if item.is_loaded() {
        let item_effect_datas = item.get_effect_datas().unwrap();
        for eff_id in item_effect_datas.keys() {
            let mut stopping_effects = Vec::with_capacity(item_effect_datas.len());
            match ss_data.src.get_a_effect(eff_id) {
                Some(e) if states.contains(&e.state) => stopping_effects.push(e.clone()),
                _ => (),
            }
            if !stopping_effects.is_empty() {
                notify::effects_stopped(item, &stopping_effects, ss_data.items, ss_data.calc);
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
