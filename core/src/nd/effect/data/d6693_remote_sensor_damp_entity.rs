use super::shared::add_damp_mods;
use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectModProjAttrsGetter, NEffectProjMultGetter},
};

const EFFECT_EID: EEffectId = EEffectId::REMOTE_SENSOR_DAMP_ENTITY;
const EFFECT_AID: AEffectId = AEffectId::REMOTE_SENSOR_DAMP_ENTITY;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| add_damp_mods(EFFECT_AID, a_effect)),
        modifier_proj_attrs_getter: Some(NEffectModProjAttrsGetter::Simple),
        modifier_proj_mult_getter: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
        ..
    }
}
