use crate::{
    ac, ad, ec, ed,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::TGT_DISINTEGRATOR_ATTACK;
const A_EFFECT_ID: ad::AEffectId = ac::effects::TGT_DISINTEGRATOR_ATTACK;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            charge: Some(NEffectCharge::Loaded(NEffectChargeDepl::ChargeRate)),
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            ..
        },
        ..
    }
}
