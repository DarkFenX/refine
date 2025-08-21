use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::{
            proj_mult::{get_full_mod_proj_attrs, get_noapp_full_proj_mult},
            wd::update_effect_td,
        },
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_TRACKING_DISRUPTOR;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_TRACKING_DISRUPTOR;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(|a_effect| update_effect_td(A_EFFECT_ID, a_effect)),
        modifier_proj_attrs_getter: Some(get_full_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_noapp_full_proj_mult),
            ..
        },
        ..
    }
}
