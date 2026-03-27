use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectProjMultGetter, NModProjAttrsGetter, effect::data::shared::mods::add_web_mods},
};

const EFFECT_EID: EEffectId = EEffectId::REMOTE_WEBIFIER_ENTITY;
const EFFECT_AID: AEffectId = AEffectId::REMOTE_WEBIFIER_ENTITY;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| add_web_mods(EFFECT_AID, a_effect)),
        modifier_proj_attrs_getter: Some(NModProjAttrsGetter::Simple),
        modifier_proj_mult_getter: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
        ..
    }
}
