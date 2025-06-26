use crate::{
    ac, ad, ec, ed,
    ntt::{NttEffect, eff::shared::web::update_effect},
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::REMOTE_WEBIFIER_FALLOFF;
const A_EFFECT_ID: ad::AEffectId = ac::effects::REMOTE_WEBIFIER_FALLOFF;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        custom_fn_adg: Some(|a_data| update_effect(a_data, A_EFFECT_ID)),
        ..
    }
}
