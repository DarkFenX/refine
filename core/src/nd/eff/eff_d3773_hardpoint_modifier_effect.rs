use crate::{
    ac,
    ad::{AEffect, AEffectId},
    ec,
    ed::EEffectId,
    nd::{NEffect, eff::shared::subsystem_mods::make_modifier},
};

const E_EFFECT_ID: EEffectId = ec::effects::HARDPOINT_MODIFIER_EFFECT;
const A_EFFECT_ID: AEffectId = ac::effects::HARDPOINT_MODIFIER_EFFECT;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: hardpoint modifier effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.push(make_modifier(
        ac::attrs::TURRET_HARDPOINT_MODIFIER,
        ac::attrs::TURRET_SLOTS_LEFT,
    ));
    a_effect.mods.push(make_modifier(
        ac::attrs::LAUNCHER_HARDPOINT_MODIFIER,
        ac::attrs::LAUNCHER_SLOTS_LEFT,
    ));
}
