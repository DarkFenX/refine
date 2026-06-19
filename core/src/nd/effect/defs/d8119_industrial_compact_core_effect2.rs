use crate::{
    ad::{AEffectId},
    nd::NEffect,
};
use crate::nd::NEffectDuration;

const EFFECT_AID: AEffectId = AEffectId::INDUSTRIAL_COMPACT_CORE_EFFECT2;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        // Subcapital industrial cores have no modifiers which disallow cloaking, but still disallow
        // it, as tested on Singularity on 2026-06-14.
        disallows_cloak: Some(NEffectDuration::Effect),
        // The effect have modifier to transfer gate scramble strength to ship's gate scramble
        // status, but its strength is not sufficient to block jumps of subcapitals. Nevertheless,
        // ships with a subcapital industrial core cannot jump gates, as tested on Singularity on
        // 2026-06-14.
        disallows_jump_gate: Some(NEffectDuration::Effect),
        // Orca/porpoise have low enough mass to be below one-pass-limit mass of some wormholes even
        // with 10x penalty from core. Despite that, they cannot jump wormholes with core active, as
        // tested on Singularity on 2026-06-14.
        disallows_jump_wh: Some(NEffectDuration::Effect),
        ..
    }
}
