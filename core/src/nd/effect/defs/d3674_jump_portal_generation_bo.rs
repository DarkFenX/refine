use super::shared::{mk_cannot_cloak_mod, mk_disallow_assistance_mod};
use crate::{
    ad::{AEffect, AEffectId},
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::JUMP_PORTAL_GENERATION_BO;
const EFFECT_AID: AEffectId = AEffectId::JUMP_PORTAL_GENERATION_BO;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
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
        .extend([mk_disallow_assistance_mod(), mk_cannot_cloak_mod()]);
}
