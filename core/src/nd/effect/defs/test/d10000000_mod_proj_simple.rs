use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NEffect, NEffectProjGetter},
};

const EFFECT_AID: AEffectId = AEffectId::from_eid(EEffectId::from_i32(10_000_000));

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        modifier_proj: Some(NEffectProjGetter::GenericRangeSimpleSts),
        ..
    }
}
