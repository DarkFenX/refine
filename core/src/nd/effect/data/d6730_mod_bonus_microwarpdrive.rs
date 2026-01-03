use crate::{
    ac,
    ad::{AEffect, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::mods::{add_prop_speed_mod, mk_mwd_sig_mod, mk_prop_mass_mod},
    },
};

const EFFECT_EID: EEffectId = ec::effects::MOD_BONUS_MICROWARPDRIVE;
const EFFECT_AID: AEffectId = ac::effects::MOD_BONUS_MICROWARPDRIVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        calc_customizer: Some(add_prop_speed_mod),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: MWD effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([mk_prop_mass_mod(), mk_mwd_sig_mod()]);
}
