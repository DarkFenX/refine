use crate::{
    ac, ad, ec, ed,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_unrestricted_s2s},
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::PROJECTILE_FIRED;
const A_EFFECT_ID: ad::AEffectId = ac::effects::PROJECTILE_FIRED;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NEffectHc {
            charge: Some(NEffectCharge::Loaded(NEffectChargeDepl::ChargeRate {
                can_run_uncharged: false,
            })),
            get_proj_mult: Some(get_proj_mult_normal_unrestricted_s2s),
            ..
        },
        ..
    }
}
