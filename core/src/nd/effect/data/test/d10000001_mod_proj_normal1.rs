use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::proj_mult::{get_full_mod_proj_attrs, get_full_noapp_proj_mult},
    },
};

const EFFECT_EID: EEffectId = EEffectId::from_i32(10_000_001);
const EFFECT_AID: AEffectId = AEffectId::from_eid(EFFECT_EID);

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        modifier_proj_attrs_getter: Some(get_full_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_full_noapp_proj_mult),
        ..
    }
}
