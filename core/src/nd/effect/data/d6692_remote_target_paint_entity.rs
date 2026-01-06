use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::{
            mods::add_tp_mods,
            proj_mult::{get_simple_mod_proj_attrs, get_simple_s2s_noapp_proj_mult},
        },
    },
};

const EFFECT_EID: EEffectId = EEffectId::REMOTE_TARGET_PAINT_ENTITY;
const EFFECT_AID: AEffectId = AEffectId::REMOTE_TARGET_PAINT_ENTITY;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| add_tp_mods(EFFECT_AID, a_effect)),
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_simple_s2s_noapp_proj_mult),
        ..
    }
}
