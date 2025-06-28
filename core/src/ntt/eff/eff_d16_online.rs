use crate::{ac, ad, ec, ed, ntt::NttEffect};

const E_EFFECT_ID: ed::EEffectId = ec::effects::ONLINE;
const A_EFFECT_ID: ad::AEffectId = ac::effects::ONLINE;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&A_EFFECT_ID) {
        Some(effect) => match effect.state {
            ad::AState::Online => tracing::info!("\"online\" effect {A_EFFECT_ID} category did not need fixing"),
            _ => effect.state = ad::AState::Online,
        },
        None => tracing::info!("\"online\" effect {A_EFFECT_ID} is not found for fixing"),
    }
}
