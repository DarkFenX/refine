use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectBuffScope, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_simple_mod_proj_attrs, get_simple_s2s_noapp_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::MOD_TITAN_EFFECT_GENERATOR;
const A_EFFECT_ID: AEffectId = ac::effects::MOD_TITAN_EFFECT_GENERATOR;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            default_attrs: Some(AEffectBuffScope::Projected(ac::itemlists::SHIPS)),
            ..
        }),
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_simple_s2s_noapp_proj_mult),
            ..
        },
        ..
    }
}
