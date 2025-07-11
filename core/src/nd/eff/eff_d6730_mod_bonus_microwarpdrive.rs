use crate::{
    ac, ad, ec, ed,
    nd::{
        NEffect, NEffectHc,
        eff::shared::prop_mods::{calc_add_custom_modifier, mk_a_modifier_mass, mk_a_modifier_sig},
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::MOD_BONUS_MICROWARPDRIVE;
const A_EFFECT_ID: ad::AEffectId = ac::effects::MOD_BONUS_MICROWARPDRIVE;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        hc: NEffectHc {
            calc_custom_fn: Some(calc_add_custom_modifier),
            ..
        },
        ..
    }
}

fn update_effect(a_effect: &mut ad::AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: MWD effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.push(mk_a_modifier_mass());
    a_effect.mods.push(mk_a_modifier_sig());
    a_effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
}
