use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{
            proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_restricted_s2s},
            wd::update_effect_gd,
        },
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_GUIDANCE_DISRUPTOR;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_GUIDANCE_DISRUPTOR;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(|a_effect| update_effect_gd(A_EFFECT_ID, a_effect)),
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NEffectHc {
            proj_mult_getter: Some(get_proj_mult_normal_restricted_s2s),
            ..
        },
        ..
    }
}
