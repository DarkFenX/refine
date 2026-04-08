use super::shared::mk_can_cloak_mod;
use crate::{
    ad::{AEffect, AEffectId},
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::MOD_BONUS_NETWORKED_SENSOR_ARRAY;
const EFFECT_AID: AEffectId = AEffectId::MOD_BONUS_NETWORKED_SENSOR_ARRAY;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    // Not tested, assumed to work similar to integrated sensor array
    a_effect.modifiers.insert(mk_can_cloak_mod());
}
