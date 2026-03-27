use super::shared::add_gd_mods;
use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectModProjAttrsGetter, NEffectProjMultGetter},
};

const EFFECT_EID: EEffectId = EEffectId::SHIP_MOD_GUIDANCE_DISRUPTOR;
const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_GUIDANCE_DISRUPTOR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| add_gd_mods(EFFECT_AID, a_effect)),
        modifier_proj_attrs_getter: Some(NEffectModProjAttrsGetter::Full),
        modifier_proj_mult_getter: Some(NEffectProjMultGetter::GenericRangeFullStsRestricted),
        ..
    }
}
