use crate::{
    ac, ad, ec, ed,
    ntt::{
        NttEffect, NttEffectRt,
        eff::shared::prop_mods::{calc_add_custom_modifier, mk_a_modifier_mass},
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::MOD_BONUS_AFTERBURNER;
const A_EFFECT_ID: ad::AEffectId = ac::effects::MOD_BONUS_AFTERBURNER;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(update_a_effect),
        rt: NttEffectRt {
            calc_custom_fn: Some(calc_add_custom_modifier),
            ..
        },
        ..
    }
}

fn update_a_effect(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&A_EFFECT_ID) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("AB effect {A_EFFECT_ID} has modifiers, overwriting them");
                effect.mods.clear();
            }
            effect.mods.push(mk_a_modifier_mass());
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("AB effect {A_EFFECT_ID} is not found for customization"),
    }
}
