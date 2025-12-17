use crate::{
    ac,
    ad::{AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const E_EFFECT_ID: EEffectId = ec::effects::JUMP_PORTAL_GENERATION_BO;
const A_EFFECT_ID: AEffectId = ac::effects::JUMP_PORTAL_GENERATION_BO;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
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
        op: AOp::PostAssign,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: ac::attrs::DISALLOW_ASSISTANCE,
    };
    a_effect.modifiers.push(modifier);
}
