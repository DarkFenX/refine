use crate::{
    ac, ad,
    misc::EffectMode,
    uad::{Uad, UadItem},
};

pub(in crate::sol::api::util) fn resolve_effect_status(
    item: &UadItem,
    item_a_state: ad::AState,
    a_effect: &ad::AEffectRt,
    online_running: bool,
) -> bool {
    // Ghosted items should never affect anything regardless of effect mode
    if item_a_state == ad::AState::Ghost {
        return false;
    }
    match item.get_effect_modes().get(&a_effect.ae.id) {
        EffectMode::FullCompliance => resolve_effect_status_full(item, item_a_state, a_effect, online_running),
        EffectMode::StateCompliance => item_a_state >= a_effect.ae.state,
        EffectMode::ForceRun => true,
        EffectMode::ForceStop => false,
    }
}

fn resolve_effect_status_full(
    item: &UadItem,
    item_a_state: ad::AState,
    a_effect: &ad::AEffectRt,
    online_running: bool,
) -> bool {
    match a_effect.ae.state {
        ad::AState::Ghost => unreachable!("ghost state should never reach full resolver"),
        // Offline effects require item in offline+ state, and no fitting usage chance attribute
        // (not to run booster side effects by default)
        ad::AState::Offline => item_a_state >= a_effect.ae.state && a_effect.ae.chance_attr_id.is_none(),
        // Online effects depend on 'online' effect, ignoring everything else
        ad::AState::Online => {
            // Online effect itself runs unconditionally if item is online+
            if a_effect.ae.id == ac::effects::ONLINE {
                item_a_state >= a_effect.ae.state
            // Other effects from online category rely only on "online" effect run status
            } else {
                online_running
            }
        }
        // Only default active effect is run, and only if item is in active+ state
        ad::AState::Active => {
            if a_effect.ae.state > item_a_state {
                return false;
            };
            match item.get_a_defeff_id() {
                Some(Some(defeff_id)) => defeff_id == a_effect.ae.id,
                _ => false,
            }
        }
        // No additional restrictions for overload effects except for item being overloaded
        ad::AState::Overload => item_a_state >= a_effect.ae.state,
    }
}

pub(in crate::sol::api::util) fn resolve_online_effect_status(
    uad: &Uad,
    item: &UadItem,
    item_a_state: ad::AState,
) -> bool {
    if !item.get_a_effect_datas().unwrap().contains_key(&ac::effects::ONLINE) {
        return false;
    }
    let effect = match uad.src.get_a_effect(&ac::effects::ONLINE) {
        Some(effect) => effect,
        None => return false,
    };
    resolve_effect_status(item, item_a_state, effect, false)
}
