use crate::{
    ac, ad, ec, ed,
    nd::{NEffect, eff::shared::subsystem_mods::make_modifier},
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::SLOT_MODIFIER;
const A_EFFECT_ID: ad::AEffectId = ac::effects::SLOT_MODIFIER;

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
        tracing::info!("effect {A_EFFECT_ID}: slot modifier effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect
        .mods
        .push(make_modifier(ac::attrs::HI_SLOT_MODIFIER, ac::attrs::HI_SLOTS));
    a_effect
        .mods
        .push(make_modifier(ac::attrs::MED_SLOT_MODIFIER, ac::attrs::MED_SLOTS));
    a_effect
        .mods
        .push(make_modifier(ac::attrs::LOW_SLOT_MODIFIER, ac::attrs::LOW_SLOTS));
}
