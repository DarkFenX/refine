use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        effect::data::shared::{
            mods::add_damp_mods,
            proj_mult::{get_simple_mod_proj_attrs, get_simple_s2s_noapp_proj_mult},
        },
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::REMOTE_SENSOR_DAMP_ENTITY;
const A_EFFECT_ID: AEffectId = ac::effects::REMOTE_SENSOR_DAMP_ENTITY;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(|a_effect| add_damp_mods(A_EFFECT_ID, a_effect)),
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_simple_s2s_noapp_proj_mult),
            ..
        },
        ..
    }
}
