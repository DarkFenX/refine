use super::shared::add_web_mods;
use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectProjGetter},
};

const EFFECT_EID: EEffectId = EEffectId::REMOTE_WEBIFIER_FALLOFF;
const EFFECT_AID: AEffectId = AEffectId::REMOTE_WEBIFIER_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(|a_effect| add_web_mods(EFFECT_AID, a_effect)),
        modifier_proj: Some(NEffectProjGetter::GenericRangeFullStsRestricted),
        ..
    }
}
