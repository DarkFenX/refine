use crate::{
    ad::{AEffect, AEffectId},
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::SKILL_EFFECT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, a_warnings: &mut Vec<String>) {
    // In EVE, skill effect defines skill level by calculating it from skill points and training
    // multiplier. Refine has no concept of skill points, and tracks levels directly, so it is
    // wasted effort. Make sure it does not happen
    match a_effect.modifiers.is_empty() {
        true => {
            let warning = format!("effect {EFFECT_AID}: no modifiers to clean");
            a_warnings.push(warning);
        }
        false => a_effect.modifiers.clear(),
    }
}
