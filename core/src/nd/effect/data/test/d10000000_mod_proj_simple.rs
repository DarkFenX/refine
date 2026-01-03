use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::proj_mult::{get_simple_mod_proj_attrs, get_simple_s2s_noapp_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = EEffectId::new(10_000_000);
const A_EFFECT_ID: AEffectId = E_EFFECT_ID.into();

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_simple_s2s_noapp_proj_mult),
        ..
    }
}
