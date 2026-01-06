use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::{
            mods::add_web_mods,
            proj_mult::{get_full_mod_proj_attrs, get_full_noapp_proj_mult},
        },
    },
};

const EFFECT_EID: EEffectId = EEffectId::REMOTE_WEBIFIER_FALLOFF;
const EFFECT_AID: AEffectId = AEffectId::REMOTE_WEBIFIER_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| add_web_mods(EFFECT_AID, a_effect)),
        modifier_proj_attrs_getter: Some(get_full_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_full_noapp_proj_mult),
        ..
    }
}
