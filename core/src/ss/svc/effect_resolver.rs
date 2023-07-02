use crate::{
    ad,
    consts::{effects, EffectMode},
    shr::State,
    ss::{item::SsItem, SsView},
};

pub(in crate::ss::svc) fn resolve_effect_status(
    item: &SsItem,
    item_state: State,
    effect: &ad::ArcEffect,
    online_running: bool,
) -> bool {
    // Ghost'ed items should never affect anything regardless of effect mode
    if item_state == State::Ghost {
        return false;
    }
    match item.get_effect_modes().get(&effect.id) {
        EffectMode::FullCompliance => resolve_effect_status_full(item, item_state, &effect, online_running),
        EffectMode::StateCompliance => item_state >= effect.state,
        EffectMode::ForceRun => true,
        EffectMode::ForceStop => false,
    }
}

fn resolve_effect_status_full(item: &SsItem, item_state: State, effect: &ad::ArcEffect, online_running: bool) -> bool {
    match effect.state {
        // Offline effects require item in offline+ state, and no fitting usage chance attribute
        // (not to run booster side effects by default)
        State::Offline => item_state >= effect.state && effect.chance_attr_id.is_none(),
        // Online effects depend on 'online' effect, ignoring everything else
        State::Online => {
            // Online effect itself runs unconditionally if item is online+
            if effect.id == effects::ONLINE {
                item_state >= effect.state
            // Other effects from online category rely only on "online" effect run status
            } else {
                online_running
            }
        }
        // Only default active effect is run, and only if item is in active+ state
        State::Active => {
            if effect.state > item_state {
                return false;
            };
            match item.get_defeff_id() {
                Ok(defeff_id_opt) => match defeff_id_opt {
                    Some(defeff_id) => *defeff_id == effect.id,
                    _ => false,
                },
                _ => false,
            }
        }
        // No additional restrictions for overload effects except for item being overloaded
        State::Overload => item_state >= effect.state,
        // None of effects should have ghost state, this is custom state for items
        State::Ghost => false,
    }
}

pub(in crate::ss::svc) fn resolve_online_effect_status(ss_view: &SsView, item: &SsItem, item_state: State) -> bool {
    if !item.get_effect_datas().unwrap().contains_key(&effects::ONLINE) {
        return false;
    }
    let effect = match ss_view.src.get_a_effect(&effects::ONLINE) {
        Some(effect) => effect,
        None => return false,
    };
    resolve_effect_status(item, item_state, &effect, false)
}
