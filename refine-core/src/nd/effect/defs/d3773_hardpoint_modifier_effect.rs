use super::shared::mk_subsystem_mod;
use crate::{
    ad::{AAttrId, AEffect, AEffectId},
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::HARDPOINT_MODIFIER_EFFECT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, a_warnings: &mut Vec<String>) {
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {EFFECT_AID}: hardpoint modifier effect has modifiers, overwriting them");
        a_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        mk_subsystem_mod(AAttrId::TURRET_HARDPOINT_MODIFIER, AAttrId::TURRET_SLOTS_LEFT),
        mk_subsystem_mod(AAttrId::LAUNCHER_HARDPOINT_MODIFIER, AAttrId::LAUNCHER_SLOTS_LEFT),
    ]);
}
