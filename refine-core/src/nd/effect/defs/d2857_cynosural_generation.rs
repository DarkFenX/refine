use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectDuration},
};

const EFFECT_AID: AEffectId = AEffectId::CYNOSURAL_GENERATION;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        // Cyno effect's dogma modifiers change following on-ship things:
        // - warpScrambleStatus (prevents warps, tether, docking into citadels, drive jumps)
        // - canCloak (prevents cloaking)
        // But, as tested on 2026-06-13 on Singularity, cyno disallows doing more things:
        // - gate jumping
        // - wormhole jumping
        // - docking into station
        // To force those effects, some flags are set manually below.
        disallows_jump_gate: Some(NEffectDuration::Effect),
        disallows_jump_wh: Some(NEffectDuration::Effect),
        disallows_dock: Some(NEffectDuration::Effect),
        ..
    }
}
