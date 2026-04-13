use super::shared::mk_subsystem_mod;
use crate::{
    ad::{AAttrId, AEffect, AEffectId},
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::HARDPOINT_MODIFIER_EFFECT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(internal_update_effect),
        ..
    }
}

fn internal_update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: hardpoint modifier effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        mk_subsystem_mod(AAttrId::TURRET_HARDPOINT_MODIFIER, AAttrId::TURRET_SLOTS_LEFT),
        mk_subsystem_mod(AAttrId::LAUNCHER_HARDPOINT_MODIFIER, AAttrId::LAUNCHER_SLOTS_LEFT),
    ]);
}
