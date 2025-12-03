use crate::{
    ac,
    ad::{AEffect, AEffectId},
    ec,
    ed::EEffectId,
    nd::{NEffect, effect::shared::mods::mk_subsystem_mod},
};

const E_EFFECT_ID: EEffectId = ec::effects::HARDPOINT_MODIFIER_EFFECT;
const A_EFFECT_ID: AEffectId = ac::effects::HARDPOINT_MODIFIER_EFFECT;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(internal_update_effect),
        ..
    }
}

fn internal_update_effect(a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: hardpoint modifier effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.extend([
        mk_subsystem_mod(ac::attrs::TURRET_HARDPOINT_MODIFIER, ac::attrs::TURRET_SLOTS_LEFT),
        mk_subsystem_mod(ac::attrs::LAUNCHER_HARDPOINT_MODIFIER, ac::attrs::LAUNCHER_SLOTS_LEFT),
    ]);
}
