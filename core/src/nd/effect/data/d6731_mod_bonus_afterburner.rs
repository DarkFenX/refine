use crate::{
    ad::{AEffect, AEffectId},
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::mods::{add_prop_speed_mod, mk_prop_mass_mod},
    },
};

const EFFECT_EID: EEffectId = EEffectId::MOD_BONUS_AFTERBURNER;
const EFFECT_AID: AEffectId = AEffectId::MOD_BONUS_AFTERBURNER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        calc_customizer: Some(add_prop_speed_mod),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: AB effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.insert(mk_prop_mass_mod());
}
