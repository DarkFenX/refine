use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectDuration},
};

const EFFECT_AID: AEffectId = AEffectId::MOD_BONUS_INTEGRATED_SENSOR_ARRAY;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        // Tested on 2026-04-06 on thunderdome, carrier can't cloak despite ISA having no modifiers
        // to transfer the cloak attribute
        disallows_cloak: Some(NEffectDuration::Effect),
        ..
    }
}
