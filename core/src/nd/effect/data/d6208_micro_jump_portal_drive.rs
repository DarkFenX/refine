use crate::{
    ad::{AEffect, AEffectId},
    ed::EEffectId,
    nd::{NEffect, effect::data::shared::mods::mk_mjd_sig_mod},
};

const EFFECT_EID: EEffectId = EEffectId::MICRO_JUMP_PORTAL_DRIVE;
const EFFECT_AID: AEffectId = AEffectId::MICRO_JUMP_PORTAL_DRIVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: MJFG effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.push(mk_mjd_sig_mod());
}
