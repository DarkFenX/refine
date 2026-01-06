use crate::{
    ac,
    ad::{AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AModifierSrq, AOp},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::SELF_ROF;
const EFFECT_AID: AEffectId = AEffectId::SELF_ROF;

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
        tracing::info!("effect {EFFECT_AID}: self-skillreq missile rof effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    let modifier = AEffectModifier {
        affector_attr_id: AAttrId::ROF_BONUS,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::LocSrq(AEffectLocation::Ship, AModifierSrq::SelfRef),
        affectee_attr_id: AAttrId::SPEED,
    };
    a_effect.modifiers.push(modifier);
}
