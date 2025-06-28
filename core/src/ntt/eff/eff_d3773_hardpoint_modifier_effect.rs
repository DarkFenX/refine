use crate::{
    ac, ad, ec, ed,
    ntt::{NttEffect, eff::shared::subsystem_mods::mk_modifier},
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::HARDPOINT_MODIFIER_EFFECT;
const A_EFFECT_ID: ad::AEffectId = ac::effects::HARDPOINT_MODIFIER_EFFECT;

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
                tracing::info!("hardpoint modifier effect {A_EFFECT_ID} has modifiers, overwriting them");
                effect.mods.clear();
            }
            effect.mods.push(mk_modifier(
                ac::attrs::TURRET_HARDPOINT_MODIFIER,
                ac::attrs::TURRET_SLOTS_LEFT,
            ));
            effect.mods.push(mk_modifier(
                ac::attrs::LAUNCHER_HARDPOINT_MODIFIER,
                ac::attrs::LAUNCHER_SLOTS_LEFT,
            ));
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("hardpoint modifier effect {A_EFFECT_ID} is not found for customization"),
    }
}
