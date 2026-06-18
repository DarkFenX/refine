use super::shared::mk_mjd_sig_mod;
use crate::{
    ad::{AEffect, AEffectId},
    nd::{NEffect, NEffectDuration},
};

const EFFECT_AID: AEffectId = AEffectId::MICRO_JUMP_PORTAL_DRIVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        disallows_cloak: Some(NEffectDuration::Effect),
        disallows_warp: Some(NEffectDuration::Effect),
        disallows_jump_gate: Some(NEffectDuration::Effect),
        disallows_jump_wh: Some(NEffectDuration::Effect),
        disallows_jump_drive: Some(NEffectDuration::Effect),
        disallows_dock: Some(NEffectDuration::Effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: MJFG effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.insert(mk_mjd_sig_mod());
}
