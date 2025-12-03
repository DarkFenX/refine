use crate::{
    ac,
    ad::{AEffect, AEffectId},
    ec,
    ed::EEffectId,
    nd::{NEffect, effect::data::shared::mods::mk_mjd_sig_mod},
};

const E_EFFECT_ID: EEffectId = ec::effects::MICRO_JUMP_PORTAL_DRIVE;
const A_EFFECT_ID: AEffectId = ac::effects::MICRO_JUMP_PORTAL_DRIVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: MJFG effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.push(mk_mjd_sig_mod());
}
