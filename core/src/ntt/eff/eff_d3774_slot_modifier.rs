use crate::{
    ac, ad, ec, ed,
    ntt::{NttEffect, eff::shared::subsystem_mods::mk_modifier},
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::SLOT_MODIFIER;
const A_EFFECT_ID: ad::AEffectId = ac::effects::SLOT_MODIFIER;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&A_EFFECT_ID) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("effect {A_EFFECT_ID}: slot modifier effect has modifiers, overwriting them");
                effect.mods.clear();
            }
            effect
                .mods
                .push(mk_modifier(ac::attrs::HI_SLOT_MODIFIER, ac::attrs::HI_SLOTS));
            effect
                .mods
                .push(mk_modifier(ac::attrs::MED_SLOT_MODIFIER, ac::attrs::MED_SLOTS));
            effect
                .mods
                .push(mk_modifier(ac::attrs::LOW_SLOT_MODIFIER, ac::attrs::LOW_SLOTS));
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("effect {A_EFFECT_ID}: slot modifier effect is not found for customization"),
    }
}
