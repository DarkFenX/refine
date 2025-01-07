use crate::{
    ad, ec,
    sol::{
        uad::{
            item::{SolItem, SolItemState},
            SolUad,
        },
        SolEffectMode,
    },
};

pub(in crate::sol::svc) fn resolve_effect_status(
    item: &SolItem,
    item_state: SolItemState,
    effect: &ad::ArcEffect,
    online_running: bool,
) -> bool {
    // Ghosted items should never affect anything regardless of effect mode
    if item_state == SolItemState::Ghost {
        return false;
    }
    match item.get_effect_modes().get(&effect.id) {
        SolEffectMode::FullCompliance => resolve_effect_status_full(item, item_state, &effect, online_running),
        SolEffectMode::StateCompliance => item_state >= effect.state,
        SolEffectMode::ForceRun => true,
        SolEffectMode::ForceStop => false,
    }
}

fn resolve_effect_status_full(
    item: &SolItem,
    item_state: SolItemState,
    effect: &ad::ArcEffect,
    online_running: bool,
) -> bool {
    match effect.state {
        // Offline effects require item in offline+ state, and no fitting usage chance attribute
        // (not to run booster side effects by default)
        ad::AState::Offline => item_state >= effect.state && effect.chance_attr_id.is_none(),
        // Online effects depend on 'online' effect, ignoring everything else
        ad::AState::Online => {
            // Online effect itself runs unconditionally if item is online+
            if effect.id == ec::effects::ONLINE {
                item_state >= effect.state
            // Other effects from online category rely only on "online" effect run status
            } else {
                online_running
            }
        }
        // Only default active effect is run, and only if item is in active+ state
        ad::AState::Active => {
            if effect.state > item_state {
                return false;
            };
            match item.get_defeff_id() {
                Ok(defeff_id_opt) => match defeff_id_opt {
                    Some(defeff_id) => defeff_id == effect.id,
                    _ => false,
                },
                _ => false,
            }
        }
        // No additional restrictions for overload effects except for item being overloaded
        ad::AState::Overload => item_state >= effect.state,
    }
}

pub(in crate::sol::svc) fn resolve_online_effect_status(
    uad: &SolUad,
    item: &SolItem,
    item_state: SolItemState,
) -> bool {
    if !item.get_effect_datas().unwrap().contains_key(&ec::effects::ONLINE) {
        return false;
    }
    let effect = match uad.src.get_a_effect(&ec::effects::ONLINE) {
        Some(effect) => effect,
        None => return false,
    };
    resolve_effect_status(item, item_state, effect, false)
}
