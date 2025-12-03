use crate::{
    ac,
    ad::AEffectId,
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        effect::data::shared::{
            mods::add_damp_mods,
            proj_mult::{get_full_mod_proj_attrs, get_full_noapp_proj_mult},
        },
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::STRUCT_MOD_EFFECT_REMOTE_SENSOR_DAMPENER;
const A_EFFECT_ID: AEffectId = ac::effects::STRUCT_MOD_EFFECT_REMOTE_SENSOR_DAMPENER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(|a_effect| add_damp_mods(A_EFFECT_ID, a_effect)),
        modifier_proj_attrs_getter: Some(get_full_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_full_noapp_proj_mult),
            ..
        },
        ..
    }
}
