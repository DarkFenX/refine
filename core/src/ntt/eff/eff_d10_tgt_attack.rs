use crate::{
    ac, ad, ec, ed,
    ntt::{
        NttEffect, NttEffectCharge, NttEffectChargeDepl, NttEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_unrestricted_s2s},
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::TGT_ATTACK;
const A_EFFECT_ID: ad::AEffectId = ac::effects::TGT_ATTACK;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NttEffectHc {
            charge: Some(NttEffectCharge::Loaded(NttEffectChargeDepl::Crystal)),
            get_proj_mult: Some(get_proj_mult_normal_unrestricted_s2s),
            ..
        },
        ..
    }
}
