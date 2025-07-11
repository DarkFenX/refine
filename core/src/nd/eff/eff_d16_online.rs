use crate::{ac, ad, ec, ed, nd::NEffect};

const E_EFFECT_ID: ed::EEffectId = ec::effects::ONLINE;
const A_EFFECT_ID: ad::AEffectId = ac::effects::ONLINE;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut ad::AEffect) {
    match a_effect.state {
        ad::AState::Online => {
            tracing::info!("effect {A_EFFECT_ID}: \"online\" effect category did not need fixing")
        }
        _ => a_effect.state = ad::AState::Online,
    }
}
