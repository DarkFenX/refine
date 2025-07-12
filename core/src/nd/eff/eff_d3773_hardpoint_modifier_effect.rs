use crate::{
    ac, ad, ec, ed,
    nd::{NEffect, eff::shared::subsystem_mods::mk_modifier},
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::HARDPOINT_MODIFIER_EFFECT;
const A_EFFECT_ID: ad::AEffectId = ac::effects::HARDPOINT_MODIFIER_EFFECT;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut ad::AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: hardpoint modifier effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.push(mk_modifier(
        ac::attrs::TURRET_HARDPOINT_MODIFIER,
        ac::attrs::TURRET_SLOTS_LEFT,
    ));
    a_effect.mods.push(mk_modifier(
        ac::attrs::LAUNCHER_HARDPOINT_MODIFIER,
        ac::attrs::LAUNCHER_SLOTS_LEFT,
    ));
}
