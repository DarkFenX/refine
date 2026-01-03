use crate::{
    ac,
    ad::{AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = ec::effects::JUMP_PORTAL_GENERATION;
const EFFECT_AID: AEffectId = ac::effects::JUMP_PORTAL_GENERATION;

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
    let modifier = AEffectModifier {
        affector_attr_id: ac::attrs::DISALLOW_ASSISTANCE,
        op: AOp::Add,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: ac::attrs::DISALLOW_ASSISTANCE,
    };
    a_effect.modifiers.push(modifier);
}
