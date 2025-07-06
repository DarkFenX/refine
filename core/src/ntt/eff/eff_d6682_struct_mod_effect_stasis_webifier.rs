use crate::{
    ac, ad, ec, ed,
    ntt::{
        NttEffect, NttEffectHc,
        eff::shared::{
            proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
            web::update_effect,
        },
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::STRUCT_MOD_EFFECT_STASIS_WEBIFIER;
const A_EFFECT_ID: ad::AEffectId = ac::effects::STRUCT_MOD_EFFECT_STASIS_WEBIFIER;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(|a_data| update_effect(a_data, A_EFFECT_ID)),
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NttEffectHc {
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            ..
        },
        ..
    }
}
