use super::shared::mk_cannot_cloak_mod_hardcoded;
use crate::{
    ad::{AEffect, AEffectId},
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::MOD_BONUS_SIEGE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    // Tested on 2026-04-07 on thunderdome, phoenix can't cloak with siege running, and there are no
    // attributes and modifiers to transfer either of no-cloak attributes to ship
    a_effect.modifiers.insert(mk_cannot_cloak_mod_hardcoded());
}
