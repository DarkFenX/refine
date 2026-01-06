use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::{
            mods::add_gd_mods,
            proj_mult::{get_full_mod_proj_attrs, get_full_noapp_proj_mult},
        },
    },
};

const EFFECT_EID: EEffectId = EEffectId::SHIP_MOD_GUIDANCE_DISRUPTOR;
const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_GUIDANCE_DISRUPTOR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| add_gd_mods(EFFECT_AID, a_effect)),
        modifier_proj_attrs_getter: Some(get_full_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_full_noapp_proj_mult),
        ..
    }
}
