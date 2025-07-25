use crate::{
    ac, ad, ec, ed,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_unrestricted_s2s},
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::TGT_ATTACK;
const A_EFFECT_ID: ad::AEffectId = ac::effects::TGT_ATTACK;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::Crystal),
                activates_charge: false,
            }),
            proj_mult_getter: Some(get_proj_mult_normal_unrestricted_s2s),
            ..
        },
        ..
    }
}
