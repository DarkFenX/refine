use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectProjGetter},
};

const EFFECT_EID: EEffectId = EEffectId::from_i32(10_000_000);
const EFFECT_AID: AEffectId = AEffectId::from_eid(EFFECT_EID);

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        modifier_proj: Some(NEffectProjGetter::GenericRangeSimpleSts),
        ..
    }
}
