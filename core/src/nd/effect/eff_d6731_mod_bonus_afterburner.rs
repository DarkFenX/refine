use crate::{
    ac,
    ad::{AEffect, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        effect::shared::mods::{add_prop_speed_mod, mk_prop_mass_mod},
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
            calc_customizer: Some(add_prop_speed_mod),
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
    a_effect.mods.push(mk_prop_mass_mod());
}
