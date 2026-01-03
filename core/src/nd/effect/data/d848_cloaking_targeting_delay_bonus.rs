use crate::{
    ac,
    ad::{AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AModifierSrq, AOp},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = ec::effects::CLOAKING_TARGETING_DELAY_BONUS;
const EFFECT_AID: AEffectId = ac::effects::CLOAKING_TARGETING_DELAY_BONUS;

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
        tracing::info!(
            "effect {EFFECT_AID}: self-skillreq cloaking target delay effect has modifiers, overwriting them"
        );
        a_effect.modifiers.clear();
    }
    let modifier = AEffectModifier {
        affector_attr_id: ac::attrs::CLOAKING_TARGETING_DELAY_BONUS,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::LocSrq(AEffectLocation::Ship, AModifierSrq::SelfRef),
        affectee_attr_id: ac::attrs::CLOAKING_TARGETING_DELAY,
    };
    a_effect.modifiers.push(modifier);
}
