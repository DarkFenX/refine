use crate::{
    ac,
    ad::{AEffect, AEffectId},
    ec,
    ed::EEffectId,
    nd::{NEffect, effect::data::shared::mods::mk_subsystem_mod},
};

const E_EFFECT_ID: EEffectId = ec::effects::SLOT_MODIFIER;
const A_EFFECT_ID: AEffectId = ac::effects::SLOT_MODIFIER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(internal_update_effect),
        ..
    }
}

fn internal_update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: slot modifier effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        mk_subsystem_mod(ac::attrs::HI_SLOT_MODIFIER, ac::attrs::HI_SLOTS),
        mk_subsystem_mod(ac::attrs::MED_SLOT_MODIFIER, ac::attrs::MED_SLOTS),
        mk_subsystem_mod(ac::attrs::LOW_SLOT_MODIFIER, ac::attrs::LOW_SLOTS),
    ]);
}
