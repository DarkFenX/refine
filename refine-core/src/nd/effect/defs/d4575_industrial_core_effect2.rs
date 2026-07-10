use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectDuration},
};

const EFFECT_AID: AEffectId = AEffectId::INDUSTRIAL_CORE_EFFECT2;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        // Capital industrial core has no modifiers which disallow cloaking, but still disallows it,
        // as tested on Singularity on 2026-06-14.
        disallows_cloak: Some(NEffectDuration::Effect),
        ..
    }
}
