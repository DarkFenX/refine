use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::proj_mult::{get_simple_mod_proj_attrs, get_simple_s2s_noapp_proj_mult},
    },
};

const EFFECT_EID: EEffectId = EEffectId::new(10_000_000);
const EFFECT_AID: AEffectId = EFFECT_EID.into();

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_simple_s2s_noapp_proj_mult),
        ..
    }
}
