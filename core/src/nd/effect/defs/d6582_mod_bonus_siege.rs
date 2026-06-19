use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectDuration},
};

const EFFECT_AID: AEffectId = AEffectId::MOD_BONUS_SIEGE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        // Siege module has no modifiers which disallow cloaking, but still disallows cloaking, as
        // tested on Singularity on 2026-06-14.
        disallows_cloak: Some(NEffectDuration::Effect),
        ..
    }
}
