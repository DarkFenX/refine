use super::shared::add_tp_mods;
use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectProjGetter},
};

const EFFECT_EID: EEffectId = EEffectId::REMOTE_TARGET_PAINT_FALLOFF;
const EFFECT_AID: AEffectId = AEffectId::REMOTE_TARGET_PAINT_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| add_tp_mods(EFFECT_AID, a_effect)),
        modifier_proj: Some(NEffectProjGetter::GenericRangeFullStsRestricted),
        ..
    }
}
