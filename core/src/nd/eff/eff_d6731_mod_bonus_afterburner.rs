use crate::{
    ac,
    ad::{AEffect, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::prop_mods::{calc_add_custom_modifier, mk_a_modifier_mass},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::MOD_BONUS_AFTERBURNER;
const A_EFFECT_ID: AEffectId = ac::effects::MOD_BONUS_AFTERBURNER;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        hc: NEffectHc {
            calc_customizer: Some(calc_add_custom_modifier),
            ..
        },
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: AB effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.push(mk_a_modifier_mass());
}
