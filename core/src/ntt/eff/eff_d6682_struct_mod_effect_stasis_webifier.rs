use crate::{
    ac, ad, ec, ed,
    ntt::{NttEffect, eff::shared::web::update_effect},
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::STRUCT_MOD_EFFECT_STASIS_WEBIFIER;
const A_EFFECT_ID: ad::AEffectId = ac::effects::STRUCT_MOD_EFFECT_STASIS_WEBIFIER;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        custom_fn_adg: Some(|a_data| update_effect(a_data, A_EFFECT_ID)),
        ..
    }
}
