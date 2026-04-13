use super::shared::mk_can_cloak_mod;
use crate::{
    ad::{AEffect, AEffectId},
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::MOD_BONUS_INTEGRATED_SENSOR_ARRAY;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    // Tested on 2026-04-06 on thunderdome, carrier can't cloak despite ISA having no modifiers to
    // transfer the cloak attribute
    a_effect.modifiers.insert(mk_can_cloak_mod());
}
