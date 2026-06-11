use super::shared::{mk_cannot_cloak_mod_hardcoded, mk_disallow_assistance_mod_transfer};
use crate::{
    ad::{AEffect, AEffectId},
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::JUMP_PORTAL_GENERATION;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    // In EVE, it seems like modules which disallow assistance do it indirectly. Whenever they are
    // active, assistance just cannot be applied to carrying ship. In the lib, we just transfer it
    // to ship for simplicity
    a_effect
        .modifiers
        .extend([mk_disallow_assistance_mod_transfer(), mk_cannot_cloak_mod_hardcoded()]);
}
