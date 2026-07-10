use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectDuration},
};

const EFFECT_AID: AEffectId = AEffectId::MOD_BONUS_TRIAGE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        // Triage module has no modifiers which disallow cloaking, but still disallows it, as tested
        // on Singularity on 2026-06-14.
        disallows_cloak: Some(NEffectDuration::Effect),
        ..
    }
}
