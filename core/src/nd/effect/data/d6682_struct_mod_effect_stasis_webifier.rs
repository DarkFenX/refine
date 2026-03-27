use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectModProjAttrsGetter, NEffectProjMultGetter, effect::data::shared::mods::add_web_mods},
};

const EFFECT_EID: EEffectId = EEffectId::STRUCT_MOD_EFFECT_STASIS_WEBIFIER;
const EFFECT_AID: AEffectId = AEffectId::STRUCT_MOD_EFFECT_STASIS_WEBIFIER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| add_web_mods(EFFECT_AID, a_effect)),
        modifier_proj_attrs_getter: Some(NEffectModProjAttrsGetter::Simple),
        modifier_proj_mult_getter: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
        ..
    }
}
