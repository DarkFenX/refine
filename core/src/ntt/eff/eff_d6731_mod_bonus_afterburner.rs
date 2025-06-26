use crate::{
    ac, ad, ec, ed,
    ntt::{NttEffect, eff::shared::prop_mods::mk_mass_mod},
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::MOD_BONUS_AFTERBURNER;
const A_EFFECT_ID: ad::AEffectId = ac::effects::MOD_BONUS_AFTERBURNER;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        custom_fn_adg: Some(update_effect),
        ..
    }
}

fn update_effect(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&A_EFFECT_ID) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("slot modifier effect {A_EFFECT_ID} has modifiers, overwriting them");
                effect.mods.clear();
            }
            effect.mods.push(mk_mass_mod());
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("slot modifier effect {A_EFFECT_ID} is not found for customization"),
    }
}
